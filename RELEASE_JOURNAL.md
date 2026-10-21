# Journal des releases — Calendar_API

<!-- release-journal-state {"dev": "6460af9589e02c5d504a22b584bf65b888b22d81", "prod": "a40acfd5a1afcf946ac60c1dbe32fbe210bb8263", "prod_tag": "v1.1.0", "staging": "dbc5e3f42b0e4c45c4a09f544955bd656afc879e", "updated": {"dev": "2026-10-21 09:07 UTC", "staging": "2026-10-21 03:20 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `a40acfd` | — |
| staging | — | `dbc5e3f` | 2026-10-21 03:20 UTC |
| dev | — | `6460af9` | 2026-10-21 09:07 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Changements de comportement
- Les stacks Docker Compose (`docker-compose.yml`, `docker-compose-integration.yml`, `docker-compose-performance.yml`, `docker-compose-security.yml`) connectent désormais le service `calendar` à Postgres avec le rôle `calendar_api` au lieu du superuser `postgres`. Une requête hors des grants de ce rôle échoue en `permission denied` (SQLSTATE 42501). Comme en staging, d'après la description de la PR. (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))

#### Technique
- Promotion vers staging et prod devenue manuelle : `cicd.yml` s'arrête à `release-dev`, nouveaux `promote-staging.yml` et `promote-prod.yml` (`workflow_dispatch`, entrée `sha` optionnelle), `renovate.json` couvre les trois fichiers, `CLAUDE.md` documente la procédure. (MAIR-494, [#191](https://github.com/mairie360/Calendar_API/pull/191))
- Pins `mairie360/CICD` portés de v4.2.2 à v4.4.3 (journal des releases MAIR-571, puis corrections de `promote-prod` pour le tag et la release, MAIR-612, [#184](https://github.com/mairie360/Calendar_API/pull/184), [#194](https://github.com/mairie360/Calendar_API/pull/194), [#196](https://github.com/mairie360/Calendar_API/pull/196), [#199](https://github.com/mairie360/Calendar_API/pull/199)).
- Tests : `mairie360_api_lib` passé de 3.4.0 à 4.2.0 et tests exécutés avec le rôle `calendar_api`, nouveau test `tests/queries/db_role.rs`. Le diff de la PR #180 est tronqué, la liste n'est donc pas exhaustive. (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))
- `.cargo/audit.toml` ignore RUSTSEC-2026-0367, 0368 et 0369 (zip 8 utilisé à la build de utoipa-swagger-ui). (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))
- Mises à jour de dépendances : utoipa-swagger-ui 10.0.2 ([#189](https://github.com/mairie360/Calendar_API/pull/189)), tokio 1.54.0 ([#186](https://github.com/mairie360/Calendar_API/pull/186)), digest de `rust:1.99-bookworm` dans `development.Dockerfile` ([#197](https://github.com/mairie360/Calendar_API/pull/197)).

## Différence dev → staging (en dev, pas encore en staging)

#### Technique
- Pins `mairie360/CICD` de v4.4.0 à v4.4.3 dans `cicd.yml`, `promote-staging.yml` et `promote-prod.yml` (MAIR-612, [#194](https://github.com/mairie360/Calendar_API/pull/194), [#196](https://github.com/mairie360/Calendar_API/pull/196), [#199](https://github.com/mairie360/Calendar_API/pull/199)). Aucun effet sur le service.
- Digest de `rust:1.99-bookworm` mis à jour dans `development.Dockerfile`, image de développement uniquement ([#197](https://github.com/mairie360/Calendar_API/pull/197)).

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- Promotion vers staging et prod devenue manuelle : nouveaux `promote-staging.yml` et `promote-prod.yml`, `renovate.json` et `CLAUDE.md` mis à jour (MAIR-494, [#191](https://github.com/mairie360/Calendar_API/pull/191)).
- Pins `mairie360/CICD` de `cicd.yml` portés jusqu'à v4.4.0 avec le journal des releases (MAIR-571, [#184](https://github.com/mairie360/Calendar_API/pull/184), [#185](https://github.com/mairie360/Calendar_API/pull/185), [#187](https://github.com/mairie360/Calendar_API/pull/187), [#188](https://github.com/mairie360/Calendar_API/pull/188), [#191](https://github.com/mairie360/Calendar_API/pull/191)).
- Tests et configuration de compose passés sur le rôle `calendar_api`, `mairie360_api_lib` 4.2.0 et exclusions `cargo audit` pour zip (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180)). Aucun changement pour l'environnement de production.
- Dépendances : tokio 1.54.0 ([#186](https://github.com/mairie360/Calendar_API/pull/186)) et utoipa-swagger-ui 10.0.2 ([#189](https://github.com/mairie360/Calendar_API/pull/189)).

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-10-21 09:07 UTC | dev | — | `6460af9` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39573578570) |
| 2026-10-21 08:30 UTC | dev | — | `6a5953c` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39568204180) |
| 2026-10-21 08:29 UTC | dev | — | `fe12f65` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39568348176) |
| 2026-10-21 03:51 UTC | dev | — | `347766b` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39540184692) |
| 2026-10-21 03:20 UTC | staging | — | `dbc5e3f` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39537392251) |
| 2026-10-21 01:44 UTC | dev | — | `dbc5e3f` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39527785266) |
| 2026-10-20 19:02 UTC | dev | — | `47e7356` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39478871912) |
| 2026-10-20 17:07 UTC | staging | — | `3b4668b` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39315820782) |
| 2026-10-19 20:09 UTC | dev | — | `3b4668b` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39315820782) |
| 2026-10-19 19:35 UTC | dev | — | `31022c6` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39307846830) |
| 2026-10-19 19:28 UTC | dev | — | `aeb7df2` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39307831941) |
| 2026-10-19 18:38 UTC | dev | — | `58ba995` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39300959456) |
<!-- release-journal-history:end -->
