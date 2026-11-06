# Journal des releases — Calendar_API

<!-- release-journal-state {"dev": "ef34dc5d4254ad11e11192a5c0cf8a6418c5ba53", "prod": "dbc5e3f42b0e4c45c4a09f544955bd656afc879e", "prod_tag": "v1.2.0", "staging": "dbc5e3f42b0e4c45c4a09f544955bd656afc879e", "updated": {"dev": "2026-11-06 03:12 UTC", "prod": "2026-10-21 09:11 UTC", "staging": "2026-10-21 03:20 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.2.0 | `dbc5e3f` | 2026-10-21 09:11 UTC |
| staging | — | `dbc5e3f` | 2026-10-21 03:20 UTC |
| dev | — | `ef34dc5` | 2026-11-06 03:12 UTC |

## Ajouté sur main depuis la dernière release prod (v1.2.0)

- chore(deps): pin node.js to 01d566e (#219) ([#219](https://github.com/mairie360/Calendar_API/pull/219))
- feat: MAIR-497 GDPR - Usage simulation and AI review of logs and database before prod ([#216](https://github.com/mairie360/Calendar_API/pull/216))
- chore(deps): update rust:1.99-trixie docker digest to 4c4c8ec (#218) ([#218](https://github.com/mairie360/Calendar_API/pull/218))
- chore(deps): update rust dependencies (#204) ([#204](https://github.com/mairie360/Calendar_API/pull/204))
- fix: MAIR-680 [Audit 23/10] Toutes les APIs et BFFs : remplacer le secret JWT HS256 partagé par 12 services (A1) ([#214](https://github.com/mairie360/Calendar_API/pull/214))
- chore(deps): pin dependencies (#210) ([#210](https://github.com/mairie360/Calendar_API/pull/210))
- ci: MAIR-771 APIs, BFFs, Database : monter CICD en v5.2.2 pour activer les contrôles RGPD (#212) ([#212](https://github.com/mairie360/Calendar_API/pull/212))
- fix: MAIR-741 APIs : passer l'image d'exécution en distroless Debian 13 (CVE-2026-84782) (#208) ([#208](https://github.com/mairie360/Calendar_API/pull/208))
- feat: MAIR-288 Définir la matrice d'accès des APIs et la vérifier depuis l'OpenAPI (#161) ([#161](https://github.com/mairie360/Calendar_API/pull/161))
- fix: MAIR-290 Détecter les données personnelles dans les journaux avec un test du marqueur (#153) ([#153](https://github.com/mairie360/Calendar_API/pull/153))
- ci: MAIR-295 Ajouter une checklist RGPD aux PR et des rappels d'échéances automatiques (#157) ([#157](https://github.com/mairie360/Calendar_API/pull/157))
- chore(deps): update rust:1.99-bookworm docker digest to 48b7f43 (#203) ([#203](https://github.com/mairie360/Calendar_API/pull/203))
- chore(deps): update rust:1.99-bookworm docker digest to edaa2a8 (#202) ([#202](https://github.com/mairie360/Calendar_API/pull/202))
- chore(deps): update rust crate tokio to v1.54.1 (#200) ([#200](https://github.com/mairie360/Calendar_API/pull/200))
- ci: MAIR-612 Release prod: align internal versions and deploy (#199) ([#199](https://github.com/mairie360/Calendar_API/pull/199))
- chore(deps): update rust:1.99-bookworm docker digest to 5c1a173 (#197) ([#197](https://github.com/mairie360/Calendar_API/pull/197))
- ci: MAIR-612 Release prod: align internal versions and deploy (#196) ([#196](https://github.com/mairie360/Calendar_API/pull/196))
- ci: MAIR-612 Release prod: align internal versions and deploy (#194) ([#194](https://github.com/mairie360/Calendar_API/pull/194))

## Différence dev → staging (en dev, pas encore en staging)

- chore(deps): pin node.js to 01d566e (#219) ([#219](https://github.com/mairie360/Calendar_API/pull/219))
- feat: MAIR-497 GDPR - Usage simulation and AI review of logs and database before prod ([#216](https://github.com/mairie360/Calendar_API/pull/216))
- chore(deps): update rust:1.99-trixie docker digest to 4c4c8ec (#218) ([#218](https://github.com/mairie360/Calendar_API/pull/218))
- chore(deps): update rust dependencies (#204) ([#204](https://github.com/mairie360/Calendar_API/pull/204))
- fix: MAIR-680 [Audit 23/10] Toutes les APIs et BFFs : remplacer le secret JWT HS256 partagé par 12 services (A1) ([#214](https://github.com/mairie360/Calendar_API/pull/214))
- chore(deps): pin dependencies (#210) ([#210](https://github.com/mairie360/Calendar_API/pull/210))
- ci: MAIR-771 APIs, BFFs, Database : monter CICD en v5.2.2 pour activer les contrôles RGPD (#212) ([#212](https://github.com/mairie360/Calendar_API/pull/212))
- fix: MAIR-741 APIs : passer l'image d'exécution en distroless Debian 13 (CVE-2026-84782) (#208) ([#208](https://github.com/mairie360/Calendar_API/pull/208))
- feat: MAIR-288 Définir la matrice d'accès des APIs et la vérifier depuis l'OpenAPI (#161) ([#161](https://github.com/mairie360/Calendar_API/pull/161))
- fix: MAIR-290 Détecter les données personnelles dans les journaux avec un test du marqueur (#153) ([#153](https://github.com/mairie360/Calendar_API/pull/153))
- ci: MAIR-295 Ajouter une checklist RGPD aux PR et des rappels d'échéances automatiques (#157) ([#157](https://github.com/mairie360/Calendar_API/pull/157))
- chore(deps): update rust:1.99-bookworm docker digest to 48b7f43 (#203) ([#203](https://github.com/mairie360/Calendar_API/pull/203))
- chore(deps): update rust:1.99-bookworm docker digest to edaa2a8 (#202) ([#202](https://github.com/mairie360/Calendar_API/pull/202))
- chore(deps): update rust crate tokio to v1.54.1 (#200) ([#200](https://github.com/mairie360/Calendar_API/pull/200))
- ci: MAIR-612 Release prod: align internal versions and deploy (#199) ([#199](https://github.com/mairie360/Calendar_API/pull/199))
- chore(deps): update rust:1.99-bookworm docker digest to 5c1a173 (#197) ([#197](https://github.com/mairie360/Calendar_API/pull/197))
- ci: MAIR-612 Release prod: align internal versions and deploy (#196) ([#196](https://github.com/mairie360/Calendar_API/pull/196))
- ci: MAIR-612 Release prod: align internal versions and deploy (#194) ([#194](https://github.com/mairie360/Calendar_API/pull/194))

## Différence staging → prod (en staging, pas encore en prod)

_Rien._

> Résumé Claude indisponible pour ce passage : listes brutes des commits.

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-11-06 03:12 UTC | dev | — | `ef34dc5` | [run](https://github.com/mairie360/Calendar_API/actions/runs/42154149929) |
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
