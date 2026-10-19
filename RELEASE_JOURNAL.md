# Journal des releases — Calendar_API

<!-- release-journal-state {"dev": "31022c602d9e98664b3a0431c1dc09bf6f5963ae", "prod": "a40acfd5a1afcf946ac60c1dbe32fbe210bb8263", "prod_tag": "v1.1.0", "staging": "a40acfd5a1afcf946ac60c1dbe32fbe210bb8263", "updated": {"dev": "2026-10-19 19:35 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `a40acfd` | — |
| staging | — | `a40acfd` | — |
| dev | — | `31022c6` | 2026-10-19 19:35 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Technique
- Les tests et les stacks `docker-compose*.yml` connectent désormais le service `calendar` avec le rôle Postgres `calendar_api` au lieu du superuser, comme en staging. Une requête hors des grants de `api_grants.sql` échoue donc dès les tests (`42501`). Ce changement ajoute aussi un test de non-régression `tests/queries/db_role.rs`, passe `mairie360_api_lib` de 3.4.0 à 4.2.0 sans effet annoncé sur Calendar, et ignore dans `.cargo/audit.toml` les avis `zip` qui ne concernent que la compilation de Swagger UI. (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))
- 4 mises à jour de dépendances : `tokio` 1.54.0, et le workflow `mairie360/CICD` en v4.2.2, v4.3.0 puis v4.3.1. ([#186](https://github.com/mairie360/Calendar_API/pull/186), [#181](https://github.com/mairie360/Calendar_API/pull/181), [#185](https://github.com/mairie360/Calendar_API/pull/185), [#187](https://github.com/mairie360/Calendar_API/pull/187))

## Différence dev → staging (en dev, pas encore en staging)

#### Technique
- Les tests et les stacks `docker-compose*.yml` connectent désormais le service `calendar` avec le rôle Postgres `calendar_api` au lieu du superuser, comme en staging. Une requête hors des grants de `api_grants.sql` échoue donc dès les tests (`42501`). Ce changement ajoute aussi un test de non-régression `tests/queries/db_role.rs`, passe `mairie360_api_lib` de 3.4.0 à 4.2.0 sans effet annoncé sur Calendar, et ignore dans `.cargo/audit.toml` les avis `zip` qui ne concernent que la compilation de Swagger UI. (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))
- 4 mises à jour de dépendances : `tokio` 1.54.0, et le workflow `mairie360/CICD` en v4.2.2, v4.3.0 puis v4.3.1. ([#186](https://github.com/mairie360/Calendar_API/pull/186), [#181](https://github.com/mairie360/Calendar_API/pull/181), [#185](https://github.com/mairie360/Calendar_API/pull/185), [#187](https://github.com/mairie360/Calendar_API/pull/187))

## Différence staging → prod (en staging, pas encore en prod)

_Rien._

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-10-19 19:35 UTC | dev | — | `31022c6` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39307846830) |
| 2026-10-19 19:28 UTC | dev | — | `aeb7df2` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39307831941) |
| 2026-10-19 18:38 UTC | dev | — | `58ba995` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39300959456) |
<!-- release-journal-history:end -->
