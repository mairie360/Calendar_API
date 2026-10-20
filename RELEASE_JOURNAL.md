# Journal des releases — Calendar_API

<!-- release-journal-state {"dev": "47e73569dfce824199d0485b221918aa28202c1b", "prod": "a40acfd5a1afcf946ac60c1dbe32fbe210bb8263", "prod_tag": "v1.1.0", "staging": "3b4668bc630095c53f5734f0ae590918f0bc3d81", "updated": {"dev": "2026-10-20 19:02 UTC", "staging": "2026-10-20 17:07 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `a40acfd` | — |
| staging | — | `3b4668b` | 2026-10-20 17:07 UTC |
| dev | — | `47e7356` | 2026-10-20 19:02 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Technique
- Les tests et les stacks `docker-compose*.yml` connectent désormais le code de Calendar avec le rôle Postgres `calendar_api` au lieu du superuser, comme en staging. Une requête hors des droits de `api_grants.sql` échoue donc avec `permission denied` (`42501`). Ce changement ajoute aussi un test de non-régression `tests/queries/db_role.rs`, monte `mairie360_api_lib` de 3.4.0 à 4.2.0 (sans impact annoncé sur Calendar) et ignore dans `cargo audit` les avis `RUSTSEC-2026-0367` à `0369`, liés au script de build de `utoipa-swagger-ui`. (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))
- 6 mises à jour de dépendances :
  - le workflow `mairie360/CICD` passe de v4.2.1 à v4.3.2 en 5 commits ; il ajoute l'étape de journal des releases (MAIR-571, [#181](https://github.com/mairie360/Calendar_API/pull/181), [#185](https://github.com/mairie360/Calendar_API/pull/185), [#187](https://github.com/mairie360/Calendar_API/pull/187), [#184](https://github.com/mairie360/Calendar_API/pull/184), [#188](https://github.com/mairie360/Calendar_API/pull/188)) ;
  - `tokio` passe en v1.54.0 ([#186](https://github.com/mairie360/Calendar_API/pull/186)).

## Différence dev → staging (en dev, pas encore en staging)

#### Technique
- Mise à jour du workflow `mairie360/CICD` de v4.3.1 à v4.3.2, qui corrige l'étape de journal des releases. (MAIR-571, [#188](https://github.com/mairie360/Calendar_API/pull/188))

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- Les tests et les stacks `docker-compose*.yml` connectent désormais le code de Calendar avec le rôle Postgres `calendar_api` au lieu du superuser, comme en staging. Une requête hors des droits de `api_grants.sql` échoue donc avec `permission denied` (`42501`). Ce changement ajoute aussi un test de non-régression `tests/queries/db_role.rs`, monte `mairie360_api_lib` de 3.4.0 à 4.2.0 (sans impact annoncé sur Calendar) et ignore dans `cargo audit` les avis `RUSTSEC-2026-0367` à `0369`. (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))
- 5 mises à jour de dépendances :
  - le workflow `mairie360/CICD` passe de v4.2.1 à v4.3.1 en 4 commits (MAIR-571, [#181](https://github.com/mairie360/Calendar_API/pull/181), [#185](https://github.com/mairie360/Calendar_API/pull/185), [#187](https://github.com/mairie360/Calendar_API/pull/187), [#184](https://github.com/mairie360/Calendar_API/pull/184)) ;
  - `tokio` passe en v1.54.0 ([#186](https://github.com/mairie360/Calendar_API/pull/186)).

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-10-20 19:02 UTC | dev | — | `47e7356` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39478871912) |
| 2026-10-20 17:07 UTC | staging | — | `3b4668b` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39315820782) |
| 2026-10-19 20:09 UTC | dev | — | `3b4668b` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39315820782) |
| 2026-10-19 19:35 UTC | dev | — | `31022c6` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39307846830) |
| 2026-10-19 19:28 UTC | dev | — | `aeb7df2` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39307831941) |
| 2026-10-19 18:38 UTC | dev | — | `58ba995` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39300959456) |
<!-- release-journal-history:end -->
