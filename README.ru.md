# jira

[![crates.io](https://img.shields.io/crates/v/jira.svg?style=flat-square)](https://crates.io/crates/jira)
[![docs.rs](https://img.shields.io/docsrs/jira?style=flat-square)](https://docs.rs/jira)
[![build status](https://img.shields.io/github/actions/workflow/status/mrrefactoring/jira.rs/.github/workflows/ci.yaml?branch=master&style=flat-square)](https://github.com/MrRefactoring/jira.rs/actions/workflows/ci.yaml)
[![license](https://img.shields.io/crates/l/jira?style=flat-square)](https://github.com/MrRefactoring/jira.rs/blob/master/LICENSE)
[![MSRV](https://img.shields.io/badge/MSRV-1.91-blue?style=flat-square&logo=rust)](https://blog.rust-lang.org/)

> [English](README.md) · 🌐 **Русский**

Rust-клиент для Atlassian Jira REST API, Rust-версия [jira.js](https://github.com/MrRefactoring/jira.js).

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

`host` — это адрес сайта без пути. Путь к API указывается в каждом запросе.

Транспорт создаётся **один раз** и передаётся всем поверхностям. Под OAuth 2.0 каждый транспорт хранит свой токен, а
Atlassian ротирует refresh-токен при каждом обновлении, поэтому второй транспорт обесценит первый.

```rust,no_run
# use jira::{Auth, Client};
# fn example(client: Client) {
let jira = jira::cloud::CloudClient::new(client.clone());
let agile = jira::agile::AgileClient::new(client);
# }
```

Каждая операция — билдер: обязательные параметры передаются аргументами, необязательные задаются методами.

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

`jira::jql` собирает запрос, в котором каждое значение взято в кавычки и экранировано, поэтому пользовательский ввод
не может изменить его структуру.

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

Без `fields` поиск возвращает только идентификаторы. Системные поля типизированы, кастомные приходят под ключами
конкретного сайта, и `Extensible::custom` читает их в ваш собственный тип:

```rust,no_run
# use jira::Extensible;
# use jira::cloud::SearchAndReconcileResults;
# use serde::Deserialize;
#[derive(Deserialize)]
struct Estimation {
    #[serde(rename = "customfield_10016")]
    story_points: Option<f64>,
}

# fn example(page: SearchAndReconcileResults) -> Result<(), serde_json::Error> {
for issue in page.issues.unwrap_or_default() {
    let fields = issue.fields.unwrap_or_default();
    let estimation: Estimation = fields.custom()?;

    println!("{} — {} ({:?})", issue.key.unwrap_or_default(), fields.summary.unwrap_or_default(), estimation.story_points);
}
# Ok(())
# }
```

Запись принимает ту же структуру, `with_custom` добавляет в неё ваши поля:

```rust,no_run
# use jira::Extensible;
# use jira::cloud::{CloudClient, IssueFields, IssueTypeDetails, IssueUpdateDetails, Project};
# use serde::Serialize;
# #[derive(Serialize)]
# struct Estimation {
#     #[serde(rename = "customfield_10016", skip_serializing_if = "Option::is_none")]
#     story_points: Option<f64>,
# }
# async fn example(jira: &CloudClient) -> jira::Result<()> {
let fields = IssueFields {
    project: Some(Project { key: Some("PROJ".into()), ..Default::default() }),
    issuetype: Some(IssueTypeDetails { name: Some("Task".into()), ..Default::default() }),
    summary: Some("Ротировать ключ подписи".into()),
    ..Default::default()
}
.with_custom(Estimation { story_points: Some(5.0) })?;

let created = jira
    .issues()
    .create_issue(IssueUpdateDetails { fields: Some(fields), ..Default::default() })
    .send()
    .await?;
# Ok(())
# }
```

Ключ, который уже есть в структуре, например `summary`, отклоняется. `with` задаёт один ключ.

`stream` проходит поиск по токену страниц до последней страницы:

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

`stream` есть у каждого постраничного списка, включая проекты, пользователей, фильтры, дашборды, доски, спринты и
очереди:

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

Любая неудача — это `jira::Error`. Её предикаты сами читают HTTP-статус и код ошибки OAuth, поэтому проверяйте их:

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
| `is_auth` | 401: учётные данные отсутствуют, истекли или отклонены |
| `is_scope` | 401 из-за скоупа, который приложение не запрашивало; обновление токена не поможет |
| `is_forbidden` | 403: аутентификация пройдена, но доступа нет |
| `is_not_found` | 404: ресурса нет или он вам не виден |
| `is_rate_limit` | 429: смотрите `retry_after()` |
| `is_server` | 5xx |
| `is_network` | HTTP-ответа не было вовсе |
| `is_oauth` | токен-эндпойнт отказал или cloud id не разрешился |
| `is_config` | клиент так работать не может |
| `is_schema_mismatch` | 2xx, тело которого не то, что описывает тип |

Отказ в учётных данных через `X-Seraph-LoginReason` в ответе `200` тоже сообщается как ошибка аутентификации.

## Повторы

Повторы по умолчанию выключены. Включённые, они повторяют временные сбои транспорта и ответы 502, 503 и 504, но
никогда не повторяют 4xx, 429 и 500:

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

Чтобы отменить запрос, уроните его future или оберните в `tokio::time::timeout`. Прокси, таймауты и остальные
настройки транспорта задаются через собственный `reqwest::Client`:

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

По одной фиче на поверхность API; невключённая поверхность не компилируется.

| Флаг | Поверхность |
|---|---|
| `cloud` (по умолчанию) | Jira Cloud platform: задачи, проекты, поля, воркфлоу |
| `agile` | Jira Agile: доски, спринты, бэклог |
| `service-desk` | Jira Service Management |
| `server` | Jira Data Center: платформа и Agile в одной поверхности |
| `service-desk-server` | Jira Service Management Data Center |
| `assets` / `assets-server` | Assets в Cloud и в Data Center |
| `admin` | Администрирование организации |
| `teams` | Teams |
| `user-management` / `user-provisioning` | Управление пользователями и SCIM-провижининг |
| `webhooks` | Типы событий и полезной нагрузки, проверка подписи доставок |

Остальные фичи, все выключены по умолчанию:

| Фича | Что добавляет |
|---|---|
| `chrono` | Каждое поле `date-time` становится `Option<chrono::DateTime<Utc>>`; нераспознанное значение становится `None` |
| `tracing` | Спан уровня `DEBUG` на каждый запрос и событие на каждую попытку; учётные данные и тела не записываются |
| `audit` | Собирает поля ответа, которые сгенерированные типы не описывают |

`chrono` меняет типы полей, а cargo объединяет фичи во всей сборке, поэтому включайте её в приложениях, а не в
библиотеках.

## Другие продукты

- [jira.js](https://github.com/MrRefactoring/jira.js): те же API для Node.js и браузеров
- [confluence.js](https://github.com/MrRefactoring/confluence.js)
- [trello.js](https://github.com/MrRefactoring/trello.js)

## Лицензия

MIT
