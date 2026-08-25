# The Jira Data Center rig

`cargo xtask jira-dc up` brings up the instance the `server` suites run against, and reads its licence from
`timebomb-license.txt` beside this file. That file is not in the repository — a licence key is Atlassian's to publish
and not ours to redistribute — so a fresh checkout has to put one there. In CI it is written from the
`JIRA_DC_TIMEBOMB_LICENSE` secret.

## It has to be a Jira Software licence

Atlassian publishes [three-hour timebomb keys](https://developer.atlassian.com/platform/marketplace/timebomb-licenses-for-testing-server-apps/)
for exactly this. As of August 2026 the published **Jira Software Data Center** key is expired and Atlassian has not
replaced it: the wizard answers "This license has expired" and refuses the step.

The published **Jira Service Desk Data Center** key does install, and the rig comes up on it — but this image is
`atlassian/jira-software`, and a Service Desk licence leaves Jira Software unlicensed. `GET
/rest/api/2/applicationrole/jira-software` then answers 404 and the Scrum project template disappears, which takes
the board, the sprint and the epic with it.

Nineteen cases need that template — all of `server::agile`, all of `server::issues` and `server::crawl` — and they
stand down rather than fail when it is missing. `cargo xtask coverage server` counts what a run reached, so the cost
of an unlicensed rig is visible as a number: 242 of 444 operations on a Service Desk licence.

A Jira Software Data Center evaluation from [my.atlassian.com](https://my.atlassian.com) brings them back. Nothing in
the suites needs editing.

## The Service Management rig is not affected

`docker/jsm-dc` runs `atlassian/jira-servicemanagement`, where the published Service Desk key is the right one: it
licenses Assets and Service Desk together, and all 58 Assets operations are exercised under it.
