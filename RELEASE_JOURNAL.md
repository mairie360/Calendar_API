# Journal des releases — Calendar_API

<!-- release-journal-state {"dev": "fe12f65c487b89f664da3d4c0422fa4768d177ed", "prod": "a40acfd5a1afcf946ac60c1dbe32fbe210bb8263", "prod_tag": "v1.1.0", "staging": "dbc5e3f42b0e4c45c4a09f544955bd656afc879e", "updated": {"dev": "2026-10-21 08:30 UTC", "staging": "2026-10-21 03:20 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `a40acfd` | — |
| staging | — | `dbc5e3f` | 2026-10-21 03:20 UTC |
| dev | — | `fe12f65` | 2026-10-21 08:30 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Changements de comportement

- Les stacks Docker Compose locales et de CI (`docker-compose.yml`, `docker-compose-integration.yml`, `docker-compose-performance.yml`, `docker-compose-security.yml`) connectent désormais le service `calendar` avec le rôle Postgres `calendar_api` au lieu du superuser. Une requête hors des grants de ce rôle échoue donc en local et en CI, comme en staging et en prod. Postgres, Liquibase et le seeder gardent le superuser. (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))

#### Technique

- Tests des APIs : le code testé passe par le rôle `calendar_api` (`common::api_db()`, `AppState` sur `calendar_db_url()`), avec un test de non-régression `tests/queries/db_role.rs`. `mairie_api_lib` passe de 3.4.0 à 4.2.0 (dépendances et dev-dependencies). (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))
- Sécurité des dépendances : `rustls-webpki`, `webpki-roots` et `webpki-root-certs` sont mis à jour pour débloquer `cargo audit`, et trois advisories `zip` (RUSTSEC-2026-0367/0368/0369) sont ignorées dans `.cargo/audit.toml`. (MAIR-494, [#191](https://github.com/mairie360/Calendar_API/pull/191), [#180](https://github.com/mairie360/Calendar_API/pull/180))
- Promotion : `cicd.yml` ne va plus au-delà de `release-dev`. Les nouveaux workflows `promote-staging.yml` et `promote-prod.yml` se lancent manuellement, et chaque release réécrit `RELEASE_JOURNAL.md` sur `main` (MAIR-494, [#191](https://github.com/mairie360/Calendar_API/pull/191) ; MAIR-571, [#184](https://github.com/mairie360/Calendar_API/pull/184)).
- Dépendances : `tokio` passe en 1.54.0 ([#186](https://github.com/mairie360/Calendar_API/pull/186)) et `utoipa-swagger-ui` en 10.0.2 ([#189](https://github.com/mairie360/Calendar_API/pull/189)). Le pin `mairie360/CICD` évolue de v4.2.1 à v4.4.1 au fil des PR Renovate et de ([#181](https://github.com/mairie360/Calendar_API/pull/181), [#184](https://github.com/mairie360/Calendar_API/pull/184), [#185](https://github.com/mairie360/Calendar_API/pull/185), [#187](https://github.com/mairie360/Calendar_API/pull/187), [#188](https://github.com/mairie360/Calendar_API/pull/188), [#191](https://github.com/mairie360/Calendar_API/pull/191), [#194](https://github.com/mairie360/Calendar_API/pull/194)).
- Documentation : `CLAUDE.md` décrit la promotion, le rôle `calendar_api` des tests et la règle des grants.
- Image de dev : `development.Dockerfile` reste aligné sur le digest de `rust:1.99-bookworm` (5c1a173, [#197](https://github.com/mairie360/Calendar_API/pull/197)).

## Différence dev → staging (en dev, pas encore en staging)

#### Technique

- Image de dev : `development.Dockerfile` passe au digest `rust:1.99-bookworm` 5c1a173, aligné sur la prod (MAIR-427, [#197](https://github.com/mairie360/Calendar_API/pull/197)).
- Release : `mairie360/CICD` passe de v4.4.0 à v4.4.2 dans `cicd.yml`, `promote-staging.yml` et `promote-prod.yml`. La v4.4.2 corrige la création du tag et de la release de `promote-prod`, en échec 403 avec la v4.4.1 (MAIR-612, [#194](https://github.com/mairie360/Calendar_API/pull/194), [#196](https://github.com/mairie360/Calendar_API/pull/196)).

## Différence staging → prod (en staging, pas encore en prod)

#### Changements de comportement

- Les stacks Docker Compose locales et de CI (`docker-compose.yml`, `docker-compose-integration.yml`, `docker-compose-performance.yml`, `docker-compose-security.yml`) connectent désormais le service `calendar` avec le rôle Postgres `calendar_api` au lieu du superuser. Une requête hors des grants de ce rôle échoue donc en local et en CI, comme en staging et en prod. Postgres, Liquibase et le seeder gardent le superuser. (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))

#### Technique

- Tests des APIs : le code testé passe par le rôle `calendar_api` (`common::api_db()`, `AppState` sur `calendar_db_url()`), avec un test de non-régression `tests/queries/db_role.rs`. `mairie_api_lib` passe de 3.4.0 à 4.2.0 (dépendances et dev-dependencies). (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))
- Sécurité des dépendances : `rustls-webpki`, `webpki-roots` et `webpki-root-certs` sont mis à jour pour débloquer `cargo audit`, et trois advisories `zip` (RUSTSEC-2026-0367/0368/0369) sont ignorées dans `.cargo/audit.toml`. (MAIR-494, [#191](https://github.com/mairie360/Calendar_API/pull/191), [#180](https://github.com/mairie360/Calendar_API/pull/180))
- Promotion : `cicd.yml` ne va plus au-delà de `release-dev`. Les nouveaux workflows `promote-staging.yml` et `promote-prod.yml` se lancent manuellement, et chaque release réécrit `RELEASE_JOURNAL.md` sur `main` (MAIR-494, [#191](https://github.com/mairie360/Calendar_API/pull/191) ; MAIR-571, [#184](https://github.com/mairie360/Calendar_API/pull/184)).
- Dépendances : `tokio` passe en 1.54.0 ([#186](https://github.com/mairie360/Calendar_API/pull/186)) et `utoipa-swagger-ui` en 10.0.2 ([#189](https://github.com/mairie360/Calendar_API/pull/189)). Le pin `mairie360/CICD` passe de v4.2.2 à v4.4.0 via les PR Renovate et ([#181](https://github.com/mairie360/Calendar_API/pull/181), [#185](https://github.com/mairie360/Calendar_API/pull/185), [#187](https://github.com/mairie360/Calendar_API/pull/187), [#188](https://github.com/mairie360/Calendar_API/pull/188)).
- Documentation : `CLAUDE.md` décrit la promotion, le rôle `calendar_api` des tests et la règle des grants.

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
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
