# Journal des releases — Calendar_API

<!-- release-journal-state {"dev": "cdaf87d3ddf68ebbc2ab11cac66fed016a5b98af", "prod": "dbc5e3f42b0e4c45c4a09f544955bd656afc879e", "prod_tag": "v1.2.0", "staging": "dbc5e3f42b0e4c45c4a09f544955bd656afc879e", "updated": {"dev": "2026-11-04 02:43 UTC", "prod": "2026-10-21 09:11 UTC", "staging": "2026-10-21 03:20 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.2.0 | `dbc5e3f` | 2026-10-21 09:11 UTC |
| staging | — | `dbc5e3f` | 2026-10-21 03:20 UTC |
| dev | — | `cdaf87d` | 2026-11-04 02:43 UTC |

## Ajouté sur main depuis la dernière release prod (v1.2.0)

#### Changements de comportement
- Les tokens HS256 sont refusés (401) dès que `JWT_SECRET` n'est pas défini : l'API vérifie désormais les tokens EdDSA avec la clé publique `JWT_PUBLIC_KEY` de Core_API (`iss` `mairie360-core`, `aud` `mairie360`). Tant que `JWT_SECRET` est défini, la lib accepte encore les HS256 signés avec ce secret. Un appelant qui émet encore des HS256 doit passer aux tokens EdDSA signés par Core. (MAIR-680, [#214](https://github.com/mairie360/Calendar_API/pull/214))
- Les journaux de requêtes n'écrivent plus la query string ni le `Referer` : seuls l'adresse du client, la méthode, le chemin sans query, le statut, la taille et la durée sont journalisés. (MAIR-290, [#153](https://github.com/mairie360/Calendar_API/pull/153))

#### Technique
- Dépendances Renovate : image de développement passée de `rust:1.99-bookworm` à `rust:1.99-trixie` puis mise à jour du digest ; actix-web 4.17.0 → 4.18.1, tokio 1.54.0 → 1.54.1, log (dev) 0.4.34 → 0.4.35 ; images Docker des `docker-compose*.yml` épinglées par digest. ([#218](https://github.com/mairie360/Calendar_API/pull/218), [#204](https://github.com/mairie360/Calendar_API/pull/204), [#210](https://github.com/mairie360/Calendar_API/pull/210), [#203](https://github.com/mairie360/Calendar_API/pull/203), [#202](https://github.com/mairie360/Calendar_API/pull/202), [#200](https://github.com/mairie360/Calendar_API/pull/200), [#197](https://github.com/mairie360/Calendar_API/pull/197))
- Image d'exécution de l'API en distroless Debian 13 avec builder `rust:1.99-slim-trixie` (MAIR-741, [#208](https://github.com/mairie360/Calendar_API/pull/208)) ; matrice d'accès `access-matrix.yaml` et son test (MAIR-288, [#161](https://github.com/mairie360/Calendar_API/pull/161)), sans effet à l'exécution.
- CI : CICD monté de v4.4.0 à v5.2.2 (MAIR-612, MAIR-771, [#194](https://github.com/mairie360/Calendar_API/pull/194), [#196](https://github.com/mairie360/Calendar_API/pull/196), [#199](https://github.com/mairie360/Calendar_API/pull/199), [#212](https://github.com/mairie360/Calendar_API/pull/212)) ; checklist RGPD sur les PR (MAIR-295, [#157](https://github.com/mairie360/Calendar_API/pull/157)) ; stacks de test RGPD et tokens de test générés à chaque run (MAIR-290, MAIR-680).

## Différence dev → staging (en dev, pas encore en staging)

#### Changements de comportement
- Les tokens HS256 sont refusés (401) dès que `JWT_SECRET` n'est pas défini : l'API vérifie désormais les tokens EdDSA avec la clé publique `JWT_PUBLIC_KEY` de Core_API (`iss` `mairie360-core`, `aud` `mairie360`). Tant que `JWT_SECRET` est défini, la lib accepte encore les HS256 signés avec ce secret. Un appelant qui émet encore des HS256 doit passer aux tokens EdDSA signés par Core. (MAIR-680, [#214](https://github.com/mairie360/Calendar_API/pull/214))
- Les journaux de requêtes n'écrivent plus la query string ni le `Referer` : seuls l'adresse du client, la méthode, le chemin sans query, le statut, la taille et la durée sont journalisés. (MAIR-290, [#153](https://github.com/mairie360/Calendar_API/pull/153))

#### Technique
- Dépendances Renovate : image de développement passée de `rust:1.99-bookworm` à `rust:1.99-trixie` puis mise à jour du digest ; actix-web 4.17.0 → 4.18.1, tokio 1.54.0 → 1.54.1, log (dev) 0.4.34 → 0.4.35 ; images Docker des `docker-compose*.yml` épinglées par digest. ([#218](https://github.com/mairie360/Calendar_API/pull/218), [#204](https://github.com/mairie360/Calendar_API/pull/204), [#210](https://github.com/mairie360/Calendar_API/pull/210), [#203](https://github.com/mairie360/Calendar_API/pull/203), [#202](https://github.com/mairie360/Calendar_API/pull/202), [#200](https://github.com/mairie360/Calendar_API/pull/200), [#197](https://github.com/mairie360/Calendar_API/pull/197))
- Image d'exécution de l'API en distroless Debian 13 avec builder `rust:1.99-slim-trixie` (MAIR-741, [#208](https://github.com/mairie360/Calendar_API/pull/208)) ; matrice d'accès `access-matrix.yaml` et son test (MAIR-288, [#161](https://github.com/mairie360/Calendar_API/pull/161)), sans effet à l'exécution.
- CI : CICD monté de v4.4.0 à v5.2.2 (MAIR-612, MAIR-771, [#194](https://github.com/mairie360/Calendar_API/pull/194), [#196](https://github.com/mairie360/Calendar_API/pull/196), [#199](https://github.com/mairie360/Calendar_API/pull/199), [#212](https://github.com/mairie360/Calendar_API/pull/212)) ; checklist RGPD sur les PR (MAIR-295, [#157](https://github.com/mairie360/Calendar_API/pull/157)) ; stacks de test RGPD et tokens de test générés à chaque run (MAIR-290, MAIR-680).

## Différence staging → prod (en staging, pas encore en prod)

_Rien._

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
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
