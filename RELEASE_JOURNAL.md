# Journal des releases — Calendar_API

<!-- release-journal-state {"dev": "dbc5e3f42b0e4c45c4a09f544955bd656afc879e", "prod": "a40acfd5a1afcf946ac60c1dbe32fbe210bb8263", "prod_tag": "v1.1.0", "staging": "dbc5e3f42b0e4c45c4a09f544955bd656afc879e", "updated": {"dev": "2026-10-21 01:44 UTC", "staging": "2026-10-21 03:20 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `a40acfd` | — |
| staging | — | `dbc5e3f` | 2026-10-21 03:20 UTC |
| dev | — | `dbc5e3f` | 2026-10-21 01:44 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Changements de comportement
- Un push sur `main` ne déploie plus qu'en dev : `cicd.yml` (CICD v4.4.0) s'arrête à `release-dev`. Staging et prod se lancent à la main avec `gh workflow run promote-staging.yml -R mairie360/Calendar_API [-f sha=SHA]`, puis `promote-prod.yml`, sous approbation des environnements `Staging` / `Prod`. Staging refuse un commit sans tag `dev-<sha>` et sans run main vert ; prod refuse un commit sans `staging-<sha>`. (MAIR-494, [#191](https://github.com/mairie360/Calendar_API/pull/191))
- Les stacks compose locales (`docker-compose*.yml`) connectent le service `calendar` avec le rôle Postgres `calendar_api` au lieu du superuser `postgres` ; une requête hors des grants de `api_grants.sql` échoue donc désormais en local avec `permission denied`. (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))

#### Technique
- Versions CICD : `cicd.yml` passe de v4.2.1 à v4.4.0 (#181, #184, #185, #187, #188, #191). Les v4.3.x ajoutent l'étape « Journal des releases » qui réécrit `RELEASE_JOURNAL.md` sur `main` (MAIR-571). Renovate regroupe désormais les trois workflows dans `renovate.json` (#191).
- Dépendances : tokio 1.54.0 (#186), utoipa-swagger-ui 10.0.2 (#189), `mairie360_api_lib` 3.4.0 → 4.2.0 (#180, sans effet attendu selon la PR), rustls-webpki 0.103.16 et webpki-roots 1.0.10 pour `cargo audit` (#191). `.cargo/audit.toml` ignore RUSTSEC-2026-0367/0368/0369 (zip 8 dans le build script de utoipa-swagger-ui).
- Tests et docs : les tests de requêtes passent par `common::api_db()` (rôle `calendar_api`), les fixtures gardent le superuser, et `tests/queries/db_role.rs` vérifie le rôle et le refus `42501` (#180). CLAUDE.md décrit la promotion (#191).

## Différence dev → staging (en dev, pas encore en staging)

_Rien._

## Différence staging → prod (en staging, pas encore en prod)

#### Changements de comportement
- Un push sur `main` ne déploie plus qu'en dev : `cicd.yml` (CICD v4.4.0) s'arrête à `release-dev`. Staging et prod se lancent à la main avec `gh workflow run promote-staging.yml -R mairie360/Calendar_API [-f sha=SHA]`, puis `promote-prod.yml`, sous approbation des environnements `Staging` / `Prod`. Staging refuse un commit sans tag `dev-<sha>` et sans run main vert ; prod refuse un commit sans `staging-<sha>`. (MAIR-494, [#191](https://github.com/mairie360/Calendar_API/pull/191))
- Les stacks compose locales (`docker-compose*.yml`) connectent le service `calendar` avec le rôle Postgres `calendar_api` au lieu du superuser `postgres` ; une requête hors des grants de `api_grants.sql` échoue donc désormais en local avec `permission denied`. (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))

#### Technique
- Versions CICD : `cicd.yml` passe de v4.2.1 à v4.4.0 (#181, #184, #185, #187, #188, #191). Les v4.3.x ajoutent l'étape « Journal des releases » qui réécrit `RELEASE_JOURNAL.md` sur `main` (MAIR-571). Renovate regroupe désormais les trois workflows dans `renovate.json` (#191).
- Dépendances : tokio 1.54.0 (#186), utoipa-swagger-ui 10.0.2 (#189), `mairie360_api_lib` 3.4.0 → 4.2.0 (#180, sans effet attendu selon la PR), rustls-webpki 0.103.16 et webpki-roots 1.0.10 pour `cargo audit` (#191). `.cargo/audit.toml` ignore RUSTSEC-2026-0367/0368/0369 (zip 8 dans le build script de utoipa-swagger-ui).
- Tests et docs : les tests de requêtes passent par `common::api_db()` (rôle `calendar_api`), les fixtures gardent le superuser, et `tests/queries/db_role.rs` vérifie le rôle et le refus `42501` (#180). CLAUDE.md décrit la promotion (#191).

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-10-21 03:20 UTC | staging | — | `dbc5e3f` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39537392251) |
| 2026-10-21 01:44 UTC | dev | — | `dbc5e3f` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39527785266) |
| 2026-10-20 19:02 UTC | dev | — | `47e7356` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39478871912) |
| 2026-10-20 17:07 UTC | staging | — | `3b4668b` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39315820782) |
| 2026-10-19 20:09 UTC | dev | — | `3b4668b` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39315820782) |
| 2026-10-19 19:35 UTC | dev | — | `31022c6` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39307846830) |
| 2026-10-19 19:28 UTC | dev | — | `aeb7df2` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39307831941) |
| 2026-10-19 18:38 UTC | dev | — | `58ba995` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39300959456) |
<!-- release-journal-history:end -->
