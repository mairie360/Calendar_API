# Journal des releases — Calendar_API

<!-- release-journal-state {"dev": "dcb93d0cc938d75e9ab72859f7147c3785709703", "prod": "dbc5e3f42b0e4c45c4a09f544955bd656afc879e", "prod_tag": "v1.2.0", "staging": "dbc5e3f42b0e4c45c4a09f544955bd656afc879e", "updated": {"dev": "2026-11-04 18:25 UTC", "prod": "2026-10-21 09:11 UTC", "staging": "2026-10-21 03:20 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.2.0 | `dbc5e3f` | 2026-10-21 09:11 UTC |
| staging | — | `dbc5e3f` | 2026-10-21 03:20 UTC |
| dev | — | `dcb93d0` | 2026-11-04 18:25 UTC |

## Ajouté sur main depuis la dernière release prod (v1.2.0)

#### Changements de comportement
- Le service ne lit plus `JWT_SECRET` : il attend `JWT_PUBLIC_KEY` (clé publique PEM de Core, `mairie360_api_lib` 4.3.0). Sans `JWT_PUBLIC_KEY`, tout jeton HS256 est refusé (401). Un déploiement doit fournir la clé publique avant de passer à cette version. (MAIR-680, [#214](https://github.com/mairie360/Calendar_API/pull/214))
- Les journaux des requêtes HTTP n'affichent plus la query string ni le `Referer` : ils ne contiennent plus que l'adresse client, la méthode, le chemin sans query, le statut, la taille et la durée. (MAIR-290, [#153](https://github.com/mairie360/Calendar_API/pull/153))

#### Technique
- Image d'exécution distroless passée de `cc-debian12` à `cc-debian13`, builder et `development.Dockerfile` passés sur `rust:1.99-*-trixie`, test `mair_741_every_image_runs_on_debian_13`. (MAIR-741, [#208](https://github.com/mairie360/Calendar_API/pull/208))
- Simulation RGPD ajoutée à la CI : `gdpr-simulation.yaml`, `gdpr-journey.yaml`, `docker-compose-gdpr-simulation.yml`, `gdpr_simulation_test.sh`, et ignore des fichiers de run. Aucun effet sur le service. (MAIR-497, [#216](https://github.com/mairie360/Calendar_API/pull/216))
- Workflows CICD montés en v4.4.1, v4.4.2 puis v4.4.3 (promotions staging et prod), puis v5.2.2 pour activer les contrôles RGPD, avec `ANTHROPIC_WORKSPACE_ID` transmis à la promotion prod. (MAIR-612 [#194](https://github.com/mairie360/Calendar_API/pull/194), [#196](https://github.com/mairie360/Calendar_API/pull/196), [#199](https://github.com/mairie360/Calendar_API/pull/199) ; MAIR-771, [#212](https://github.com/mairie360/Calendar_API/pull/212))
- Checklist RGPD sur les PR (`gdpr-pr.yml`) et test de garde `tests/mair_771_cicd_version.rs`. (MAIR-295, [#157](https://github.com/mairie360/Calendar_API/pull/157) ; MAIR-771, [#212](https://github.com/mairie360/Calendar_API/pull/212))
- Matrice d'accès `access-matrix.yaml` et test de couverture de l'OpenAPI, sans changement de l'API. (MAIR-288, [#161](https://github.com/mairie360/Calendar_API/pull/161))
- Images des stacks de test et de `docker-compose.yml` épinglées par digest (Renovate, [#210](https://github.com/mairie360/Calendar_API/pull/210)). Digests de `rust:1.99` mis à jour dans `development.Dockerfile` ([#197](https://github.com/mairie360/Calendar_API/pull/197), [#202](https://github.com/mairie360/Calendar_API/pull/202), [#203](https://github.com/mairie360/Calendar_API/pull/203), [#218](https://github.com/mairie360/Calendar_API/pull/218)).
- Dépendances Rust : `tokio` 1.54.1 ([#200](https://github.com/mairie360/Calendar_API/pull/200)) ; `actix-web` et `log` selon la PR ([#204](https://github.com/mairie360/Calendar_API/pull/204), diff vide dans la source, non vérifié).
- Exception `RUSTSEC-2026-0387` ajoutée à `.cargo/audit.toml`. (MAIR-290, [#153](https://github.com/mairie360/Calendar_API/pull/153))

## Différence dev → staging (en dev, pas encore en staging)

#### Changements de comportement
- Le service ne lit plus `JWT_SECRET` : il attend `JWT_PUBLIC_KEY` (clé publique PEM de Core, `mairie360_api_lib` 4.3.0). Sans `JWT_PUBLIC_KEY`, tout jeton HS256 est refusé (401). Un déploiement doit fournir la clé publique avant de passer à cette version. (MAIR-680, [#214](https://github.com/mairie360/Calendar_API/pull/214))
- Les journaux des requêtes HTTP n'affichent plus la query string ni le `Referer` : ils ne contiennent plus que l'adresse client, la méthode, le chemin sans query, le statut, la taille et la durée. (MAIR-290, [#153](https://github.com/mairie360/Calendar_API/pull/153))

#### Technique
- Image d'exécution distroless passée de `cc-debian12` à `cc-debian13`, builder et `development.Dockerfile` passés sur `rust:1.99-*-trixie`, test `mair_741_every_image_runs_on_debian_13`. (MAIR-741, [#208](https://github.com/mairie360/Calendar_API/pull/208))
- Simulation RGPD ajoutée à la CI : `gdpr-simulation.yaml`, `gdpr-journey.yaml`, `docker-compose-gdpr-simulation.yml`, `gdpr_simulation_test.sh`, et ignore des fichiers de run. Aucun effet sur le service. (MAIR-497, [#216](https://github.com/mairie360/Calendar_API/pull/216))
- Workflows CICD montés en v4.4.1, v4.4.2 puis v4.4.3 (promotions staging et prod), puis v5.2.2 pour activer les contrôles RGPD, avec `ANTHROPIC_WORKSPACE_ID` transmis à la promotion prod. (MAIR-612 [#194](https://github.com/mairie360/Calendar_API/pull/194), [#196](https://github.com/mairie360/Calendar_API/pull/196), [#199](https://github.com/mairie360/Calendar_API/pull/199) ; MAIR-771, [#212](https://github.com/mairie360/Calendar_API/pull/212))
- Checklist RGPD sur les PR (`gdpr-pr.yml`) et test de garde `tests/mair_771_cicd_version.rs`. (MAIR-295, [#157](https://github.com/mairie360/Calendar_API/pull/157) ; MAIR-771, [#212](https://github.com/mairie360/Calendar_API/pull/212))
- Matrice d'accès `access-matrix.yaml` et test de couverture de l'OpenAPI, sans changement de l'API. (MAIR-288, [#161](https://github.com/mairie360/Calendar_API/pull/161))
- Images des stacks de test et de `docker-compose.yml` épinglées par digest (Renovate, [#210](https://github.com/mairie360/Calendar_API/pull/210)). Digests de `rust:1.99` mis à jour dans `development.Dockerfile` ([#197](https://github.com/mairie360/Calendar_API/pull/197), [#202](https://github.com/mairie360/Calendar_API/pull/202), [#203](https://github.com/mairie360/Calendar_API/pull/203), [#218](https://github.com/mairie360/Calendar_API/pull/218)).
- Dépendances Rust : `tokio` 1.54.1 ([#200](https://github.com/mairie360/Calendar_API/pull/200)) ; `actix-web` et `log` selon la PR ([#204](https://github.com/mairie360/Calendar_API/pull/204), diff vide dans la source, non vérifié).
- Exception `RUSTSEC-2026-0387` ajoutée à `.cargo/audit.toml`. (MAIR-290, [#153](https://github.com/mairie360/Calendar_API/pull/153))

## Différence staging → prod (en staging, pas encore en prod)

_Rien._

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-11-04 18:25 UTC | dev | — | `dcb93d0` | [run](https://github.com/mairie360/Calendar_API/actions/runs/41883185149) |
| 2026-11-04 02:43 UTC | dev | — | `cdaf87d` | [run](https://github.com/mairie360/Calendar_API/actions/runs/41761798868) |
| 2026-11-03 11:19 UTC | dev | — | `30217fa` | [run](https://github.com/mairie360/Calendar_API/actions/runs/41624171679) |
| 2026-11-03 01:35 UTC | dev | — | `9c9da5b` | [run](https://github.com/mairie360/Calendar_API/actions/runs/41559210628) |
| 2026-11-03 00:18 UTC | dev | — | `2030122` | [run](https://github.com/mairie360/Calendar_API/actions/runs/41550514257) |
| 2026-11-01 12:22 UTC | dev | — | `f266efe` | [run](https://github.com/mairie360/Calendar_API/actions/runs/41305127787) |
| 2026-10-22 23:06 UTC | dev | — | `564d98a` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39868682896) |
| 2026-10-22 05:42 UTC | dev | — | `f5d8257` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39728806480) |
| 2026-10-22 02:53 UTC | dev | — | `0e0506d` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39712216266) |
| 2026-10-21 09:11 UTC | prod | v1.2.0 | `dbc5e3f` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39573686812) |
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
