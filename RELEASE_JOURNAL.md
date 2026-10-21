# Journal des releases — Calendar_API

<!-- release-journal-state {"dev": "dbc5e3f42b0e4c45c4a09f544955bd656afc879e", "prod": "a40acfd5a1afcf946ac60c1dbe32fbe210bb8263", "prod_tag": "v1.1.0", "staging": "3b4668bc630095c53f5734f0ae590918f0bc3d81", "updated": {"dev": "2026-10-21 01:44 UTC", "staging": "2026-10-20 17:07 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `a40acfd` | — |
| staging | — | `3b4668b` | 2026-10-20 17:07 UTC |
| dev | — | `dbc5e3f` | 2026-10-21 01:44 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Changements de comportement
- Un push sur `main` ne lance plus que la release dev : `cicd.yml` s'arrête à `release-dev`. Staging et prod se déclenchent à la main avec `gh workflow run promote-staging.yml -R mairie360/Calendar_API [-f sha=SHA]`, puis `promote-prod.yml`. L'approbation de l'environnement reste exigée. (MAIR-494, [#191](https://github.com/mairie360/Calendar_API/pull/191))

#### Technique
- Pipeline : `cicd.yml` passe de mairie360/CICD v4.2.1 à v4.4.0, avec le journal des releases (MAIR-571) ajouté aux releases dev, staging et prod. `cargo audit` est débloqué par `rustls-webpki` 0.103.16 et `webpki-roots` / `webpki-root-certs` 1.0.10 (RUSTSEC-2026-0381/0382/0383). ([#181](https://github.com/mairie360/Calendar_API/pull/181), [#184](https://github.com/mairie360/Calendar_API/pull/184), [#185](https://github.com/mairie360/Calendar_API/pull/185), [#187](https://github.com/mairie360/Calendar_API/pull/187), [#188](https://github.com/mairie360/Calendar_API/pull/188), [#191](https://github.com/mairie360/Calendar_API/pull/191))
- Tests et docker-compose (local et CI, rien en prod) : `mairie360_api_lib` passe de 3.4.0 à 4.2.0. Les tests de requêtes et d'endpoints, comme les stacks `docker-compose*.yml`, se connectent avec le rôle `calendar_api` ; le test `tests/queries/db_role.rs` est ajouté. `.cargo/audit.toml` ignore RUSTSEC-2026-0367/0368/0369 (zip 8 via utoipa-swagger-ui). (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))
- Dépendances : utoipa-swagger-ui 10.0.1 → 10.0.2 ([#189](https://github.com/mairie360/Calendar_API/pull/189)) et tokio 1.53.3 → 1.54.0 ([#186](https://github.com/mairie360/Calendar_API/pull/186), MSRV relevée à 1.85 selon la description, diff absent).

## Différence dev → staging (en dev, pas encore en staging)

#### Changements de comportement
- Un push sur `main` ne lance plus que la release dev : `cicd.yml` s'arrête à `release-dev`. Staging et prod se déclenchent à la main avec `gh workflow run promote-staging.yml -R mairie360/Calendar_API [-f sha=SHA]`, puis `promote-prod.yml`. L'approbation de l'environnement reste exigée. (MAIR-494, [#191](https://github.com/mairie360/Calendar_API/pull/191))

#### Technique
- Pipeline : `cicd.yml` passe de mairie360/CICD v4.3.2 à v4.4.0 ([#188](https://github.com/mairie360/Calendar_API/pull/188), [#191](https://github.com/mairie360/Calendar_API/pull/191)). `cargo audit` est débloqué par les bumps de `rustls-webpki` et `webpki-roots` / `webpki-root-certs` (RUSTSEC-2026-0381/0382/0383).
- Dépendances : utoipa-swagger-ui 10.0.1 → 10.0.2 ([#189](https://github.com/mairie360/Calendar_API/pull/189), diff absent, d'après la description de la PR).

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- Pipeline : `cicd.yml` passe de mairie360/CICD v4.2.1 à v4.3.1, avec le journal des releases (MAIR-571) ajouté aux releases dev, staging et prod. ([#181](https://github.com/mairie360/Calendar_API/pull/181), [#184](https://github.com/mairie360/Calendar_API/pull/184), [#185](https://github.com/mairie360/Calendar_API/pull/185), [#187](https://github.com/mairie360/Calendar_API/pull/187))
- Tests et docker-compose (local et CI, rien en prod) : `mairie360_api_lib` passe de 3.4.0 à 4.2.0. Les tests et les stacks `docker-compose*.yml` se connectent avec le rôle `calendar_api` ; `tests/queries/db_role.rs` est ajouté. `.cargo/audit.toml` ignore RUSTSEC-2026-0367/0368/0369. (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))
- Dépendances : tokio 1.53.3 → 1.54.0 ([#186](https://github.com/mairie360/Calendar_API/pull/186), diff absent, d'après la description de la PR).

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-10-21 01:44 UTC | dev | — | `dbc5e3f` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39527785266) |
| 2026-10-20 19:02 UTC | dev | — | `47e7356` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39478871912) |
| 2026-10-20 17:07 UTC | staging | — | `3b4668b` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39315820782) |
| 2026-10-19 20:09 UTC | dev | — | `3b4668b` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39315820782) |
| 2026-10-19 19:35 UTC | dev | — | `31022c6` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39307846830) |
| 2026-10-19 19:28 UTC | dev | — | `aeb7df2` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39307831941) |
| 2026-10-19 18:38 UTC | dev | — | `58ba995` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39300959456) |
<!-- release-journal-history:end -->
