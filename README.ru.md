# jira

[![crates.io](https://img.shields.io/crates/v/jira.svg?style=flat-square)](https://crates.io/crates/jira)
[![docs.rs](https://img.shields.io/docsrs/jira?style=flat-square)](https://docs.rs/jira)
[![build status](https://img.shields.io/github/actions/workflow/status/mrrefactoring/jira.rs/.github/workflows/ci.yaml?branch=master&style=flat-square)](https://github.com/MrRefactoring/jira.rs/actions/workflows/ci.yaml)
[![license](https://img.shields.io/crates/l/jira?style=flat-square)](https://github.com/MrRefactoring/jira.rs/blob/master/LICENSE)
[![MSRV](https://img.shields.io/badge/MSRV-1.91-blue?style=flat-square&logo=rust)](https://blog.rust-lang.org/)

> [English](README.md) · 🌐 **Русский**

Rust-клиент для Atlassian Jira REST API — Rust-близнец [jira.js](https://github.com/MrRefactoring/jira.js). Транспорт
написан вручную; каждая операция и модель генерируются тем же конвейером, что и `jira.js`, поэтому разойтись библиотеки
могут только в языке. Пятьсот шестьдесят три живых теста идут против реального сайта Jira и стендов Data Center в
Docker — именно они нашли дефекты, которых не видит компилятор.

## Установка

```sh
cargo add jira
```

Нужен Rust 1.91 или новее и рантайм Tokio.

## Быстрый пример

```rust,no_run
use jira::{Auth, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .host("https://your-domain.atlassian.net")
        .auth(Auth::api_token("you@example.com", "YOUR_API_TOKEN"))
        .build()?;

    let jira = jira::cloud::CloudClient::new(client);

    let myself = jira.myself().get_current_user().send().await?;

    println!("{}", myself.display_name.unwrap_or_default());

    Ok(())
}
```

`host` — это голый адрес сайта: путь к API принадлежит запросу, а не клиенту.

Транспорт создаётся **один раз** и передаётся каждой поверхности. Под OAuth 2.0 это важно: два клиента — это два
состояния токена, а поскольку Atlassian ротирует refresh-токен при каждом обновлении, тот, кто обновится первым,
обесценит копию другого.

```rust,no_run
# use jira::{Auth, Client};
# fn example(client: Client) {
let jira = jira::cloud::CloudClient::new(client.clone());
let agile = jira::agile::AgileClient::new(client);
# }
```

Каждая операция — билдер: то, что API требует, это аргумент; то, что он лишь принимает, — метод.

```rust,no_run
# use jira::cloud::CloudClient;
# async fn example(jira: &CloudClient) -> jira::Result<()> {
let issues = jira
    .issue_search()
    .search_issues()
    .jql("project = PROJ ORDER BY created DESC")
    .max_results(50)
    .fields(["summary", "status"])
    .send()
    .await?;
# Ok(())
# }
```

## Поиск по JQL

`jql` принимает строку, а из строки, собранной через `format!`, значение умеет выбраться: одна кавычка в том, что
ввёл пользователь, закрывает литерал, и остаток читается как запрос. `jira::jql` сам расставляет операторы и
экранирует всё остальное.

```rust,no_run
# use jira::cloud::CloudClient;
use jira::jql::{field, func};

# async fn example(jira: &CloudClient, typed: &str) -> jira::Result<()> {
let query = field("project").eq("PROJ")
    .and(field("summary").contains(typed))
    .and(field("status").not_in(["Done", "Closed"]))
    .and(field("assignee").eq(func("currentUser")))
    .order_by_desc("created");

let page = jira.issue_search().search_issues().jql(query).fields(["summary", "status"]).send().await?;
# Ok(())
# }
```

В ответе удивляют две вещи. Без `fields` поиск возвращает идентификаторы и ничего больше — это умолчание самого
эндпоинта, а не решение крейта. А системные поля типизированы, тогда как кастомное поле попадает в `additional` под
тем ключом, который дал ему сайт, потому что набор кастомных полей у задачи — свойство конкретного сайта, а не API:

```rust,no_run
# use jira::cloud::SearchAndReconcileResults;
# fn example(page: SearchAndReconcileResults) {
for issue in page.issues.unwrap_or_default() {
    let fields = issue.fields.unwrap_or_default();
    let points = fields.additional.get("customfield_10016").and_then(|value| value.as_f64());

    println!("{} — {} ({points:?})", issue.key.unwrap_or_default(), fields.summary.unwrap_or_default());
}
# }
```

Та же структура идёт и на запись:

```rust,no_run
# use jira::cloud::{CloudClient, IssueFields, IssueTypeDetails, IssueUpdateDetails, Project};
# async fn example(jira: &CloudClient) -> jira::Result<()> {
let created = jira
    .issues()
    .create_issue(IssueUpdateDetails {
        fields: Some(IssueFields {
            project: Some(Project { key: Some("PROJ".into()), ..Default::default() }),
            issuetype: Some(IssueTypeDetails { name: Some("Task".into()), ..Default::default() }),
            summary: Some("Ротировать ключ подписи".into()),
            ..Default::default()
        }),
        ..Default::default()
    })
    .send()
    .await?;
# Ok(())
# }
```

Поиск листается непрозрачным токеном, а не смещением. `stream` доходит по нему до конца:

```rust,no_run
# use jira::cloud::CloudClient;
# use jira::jql::field;
use jira::futures_util::TryStreamExt;

# async fn example(jira: &CloudClient) -> jira::Result<()> {
let mut issues = jira
    .issue_search()
    .search_issues()
    .jql(field("project").eq("PROJ").order_by_desc("created"))
    .fields(["summary"])
    .stream();

while let Some(issue) = issues.try_next().await? {
    println!("{}", issue.key.unwrap_or_default());
}
# Ok(())
# }
```

Все остальные списки листаются смещением, и `stream` есть у каждого из них — проекты, пользователи, фильтры, дашборды,
доски, спринты, очереди сервис-деска — так что цикл писать не придётся никогда:

```rust,no_run
# use jira::cloud::CloudClient;
use jira::futures_util::TryStreamExt;

# async fn example(jira: &CloudClient) -> jira::Result<()> {
let mut projects = jira.projects().search_projects().stream();

while let Some(project) = projects.try_next().await? {
    println!("{}", project.key.unwrap_or_default());
}
# Ok(())
# }
```

## Аутентификация

```rust
use jira::{Auth, core::{OAuth2Config, OAuth2ServerConfig}};

// Jira Cloud: адрес учётной записи и выпущенный для неё API-токен.
let basic = Auth::api_token("you@example.com", "YOUR_API_TOKEN");

// Data Center: персональный токен доступа, который 8.14 и новее предпочитают.
let bearer = Auth::bearer("YOUR_PAT");

// Data Center: локальная учётная запись и её пароль.
let password = Auth::password("username", "password");

// Jira Cloud OAuth 2.0 (3LO). Клиент обновляет токен заранее, один раз повторяет запрос после 401
// и ходит через шлюз Atlassian, поэтому `host` не нужен.
let oauth = Auth::oauth2(OAuth2Config {
    refresh_token: Some("...".to_owned()),
    client_id: Some("...".to_owned()),
    client_secret: Some("...".to_owned()),
    ..OAuth2Config::default()
});

// OAuth 2.0 против собственного провайдера Data Center.
let oauth_server = Auth::oauth2_server(OAuth2ServerConfig {
    refresh_token: Some("...".to_owned()),
    client_id: Some("...".to_owned()),
    client_secret: Some("...".to_owned()),
    redirect_uri: Some("https://app.example.com/callback".to_owned()),
    ..OAuth2ServerConfig::default()
});
```

Atlassian ротирует refresh-токен при каждом обновлении. Сохраняйте новый через `on_token_refresh`, иначе следующее
обновление не пройдёт.

## Ошибки

Любая неудача — это `jira::Error`. Ветвитесь по предикатам, а не по варианту: они сами читают статус и код OAuth.

```rust,no_run
# use jira::{Client, Error};
# async fn example(client: &Client) {
match client.get("/rest/api/3/issue/PROJ-1").send::<serde_json::Value>().await {
    Ok(issue) => println!("{issue}"),
    Err(error) if error.is_not_found() => println!("такой задачи нет — или нет прав о ней знать"),
    Err(error) if error.is_rate_limit() => println!("подождать {:?}", error.retry_after()),
    Err(error) if error.is_reauthorization_required() => println!("грант мёртв, нужна повторная авторизация"),
    Err(error) => eprintln!("{error}"),
}
# }
```

| Предикат | Что означает |
|---|---|
| `is_auth` | 401 — учётные данные отсутствуют, истекли или отклонены |
| `is_scope` | 401 из-за скоупа, который приложение не запрашивало; обновление токена не поможет |
| `is_forbidden` | 403 — аутентифицирован, но не разрешено |
| `is_not_found` | 404 — нет или не видно вам |
| `is_rate_limit` | 429 — читайте `retry_after()` |
| `is_server` | 5xx |
| `is_network` | HTTP-ответа не было вовсе |
| `is_oauth` | токен-эндпойнт отказал или cloud id не разрешился |
| `is_config` | клиент так работать не может |
| `is_schema_mismatch` | 2xx, тело которого не то, что описывает тип |

Отклонённые учётные данные — не всегда 401: эндпойнт, разрешающий анонимный доступ, отвечает `200` телом
анонимной области видимости и сообщает об отказе только в `X-Seraph-LoginReason`. Этот заголовок тоже читается, а
ошибка сообщает статус, который реально был на проводе.

## Повторы

По умолчанию выключены. Покрывают только временные сбои транспорта и 502/503/504 — никогда 4xx, никогда 429,
никогда 500:

```rust,no_run
# use jira::{Client, RetryConfig};
# use std::time::Duration;
let client = Client::builder()
    .host("https://your-domain.atlassian.net")
    .retry(RetryConfig { max_attempts: 3, initial_delay: Duration::from_millis(500), backoff_factor: 2.0 })
    .build()?;
# Ok::<(), jira::Error>(())
```

`jira::with_retry` применяет ту же политику вокруг уже готового вызова.

## Отмена, прокси и таймауты

Отмена — средствами самого Rust: уроните future или оберните в `tokio::time::timeout`. Всё остальное настраивается
через собственный `reqwest::Client`:

```rust,no_run
# use jira::Client;
let http = reqwest::Client::builder()
    .proxy(reqwest::Proxy::all("http://proxy.internal:8080")?)
    .timeout(std::time::Duration::from_secs(30))
    .build()?;

let client = Client::builder().host("https://your-domain.atlassian.net").http_client(http).build()?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Флаги сборки

По одной фиче на поверхность API. Невключённая поверхность не компилируется, а типов в крейте больше трёх тысяч, так
что все сразу почти никому не нужны.

| Флаг | Поверхность |
|---|---|
| `cloud` (по умолчанию) | Jira Cloud platform — задачи, проекты, поля, воркфлоу |
| `agile` | Jira Agile — доски, спринты, бэклог |
| `service-desk` | Jira Service Management |
| `server` | Jira Data Center: платформа и Agile в одной поверхности |
| `service-desk-server` | Jira Service Management Data Center |
| `assets` / `assets-server` | Assets в Cloud и в Data Center |
| `admin` | Администрирование организации |
| `teams` | Teams |
| `user-management` / `user-provisioning` | Управление пользователями и SCIM-провижининг |
| `webhooks` | Типы событий и полезной нагрузки и проверка подписи, подтверждающая, что доставку прислала Jira |

Один флаг меняет не набор типов, а их вид:

| Флаг | Что меняет |
|---|---|
| `chrono` | Каждое поле `date-time` становится `Option<chrono::DateTime<Utc>>` вместо текста, в котором оно пришло |

По умолчанию выключен, и намеренно. Он меняет тип поля, а двум крейтам, зависящим от этого, разойтись в таком нельзя:
cargo объединяет их флаги, так что включение в одном месте включает его во всей сборке. В библиотеке оставляйте
выключенным и отдавайте решение приложению.

Поле остаётся необязательным даже там, где спецификация обещает значение всегда. Разбор даты может не удаться —
задокументированное Atlassian написание не является RFC 3339, а массовая очередь отвечает миллисекундами эпохи там,
где документ обещает строку, — и значение, которого читатель не узнал, становится `None`, а не роняет весь ответ.
Без флага текст сохраняется ровно таким, каким пришёл.

Ещё две — не поверхности, а инструменты. Они существуют ради собственных прогонов крейта против живого сайта и
выключенными не стоят ничего:

| Флаг | Что добавляет |
|---|---|
| `audit` | Собирает поля, которые API присылает, а сгенерированные типы не описывают |
| `coverage` | Записывает эндпоинт каждого вызова процесса — так живой прогон считает, до чего дошёл |

Ещё один — для логов вызывающего кода, а не для прогонов крейта:

| Флаг | Что добавляет |
|---|---|
| `tracing` | Спан `jira.request` вокруг каждого запроса, с методом и путём, и событие на каждую попытку — статус, а также повтор или обновление токена, когда они случаются |

Спан на уровне `DEBUG`, так что подписчик с фильтром `INFO` его не увидит, и ни учётных данных, ни тела запроса в нём нет.

## Другие продукты

- [jira.js](https://github.com/MrRefactoring/jira.js) — те же API для Node.js и браузеров
- [confluence.js](https://github.com/MrRefactoring/confluence.js)
- [trello.js](https://github.com/MrRefactoring/trello.js)

## Лицензия

MIT
