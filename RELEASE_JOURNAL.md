# Journal des releases — Calendar_API

<!-- release-journal-state {"dev": "ff6f17da72d1b1d388f98a0eaa3c8ff1ef85c50e", "prod": "89649f76be341a512b0f9874df70c7fd5b054a82", "prod_tag": "v1.3.0", "staging": "89649f76be341a512b0f9874df70c7fd5b054a82", "updated": {"dev": "2026-11-17 14:47 UTC", "prod": "2026-11-12 23:53 UTC", "staging": "2026-11-12 18:51 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.3.0 | `89649f7` | 2026-11-12 23:53 UTC |
| staging | — | `89649f7` | 2026-11-12 18:51 UTC |
| dev | — | `ff6f17d` | 2026-11-17 14:47 UTC |

## Ajouté sur main depuis la dernière release prod (v1.3.0)

- feat: MAIR-998 Test stacks follow the moving upstream tag of the stage (#246) ([#246](https://github.com/mairie360/Calendar_API/pull/246))
- chore(deps): pin actions/checkout action to 3d3c42e (#249) ([#249](https://github.com/mairie360/Calendar_API/pull/249))
- ci: MAIR-1008 Nightly rehearsal of the promotion gates (#244) ([#244](https://github.com/mairie360/Calendar_API/pull/244))
- test: MAIR-1002 Speed up the APIs' CI tests (Rust cache, one database per test, optimized dependencies) ([#248](https://github.com/mairie360/Calendar_API/pull/248))
- test: MAIR-1002 Speed up the APIs' CI tests (Rust cache, one database per test, optimized dependencies) ([#247](https://github.com/mairie360/Calendar_API/pull/247))
- chore(deps): update rust crate mairie360_api_lib to v4.4.0 (#225) ([#225](https://github.com/mairie360/Calendar_API/pull/225))
- test: MAIR-247 Translate the APIs' OpenAPI descriptions and messages into English (#242) ([#242](https://github.com/mairie360/Calendar_API/pull/242))
- chore(deps): MAIR-243 Bump quinn-udp to the patched 0.5.16 (RUSTSEC-2026-0538) (#239) ([#239](https://github.com/mairie360/Calendar_API/pull/239))
- fix: MAIR-247 Translate the APIs' OpenAPI descriptions and messages into English (#237) ([#237](https://github.com/mairie360/Calendar_API/pull/237))
- docs: MAIR-245 Rewrite the APIs' README and API.md (#227) ([#227](https://github.com/mairie360/Calendar_API/pull/227))
- chore: MAIR-243 Align the test Database version on the 3.4.0 release (#234) ([#234](https://github.com/mairie360/Calendar_API/pull/234))

## Différence dev → staging (en dev, pas encore en staging)

- feat: MAIR-998 Test stacks follow the moving upstream tag of the stage (#246) ([#246](https://github.com/mairie360/Calendar_API/pull/246))
- chore(deps): pin actions/checkout action to 3d3c42e (#249) ([#249](https://github.com/mairie360/Calendar_API/pull/249))
- ci: MAIR-1008 Nightly rehearsal of the promotion gates (#244) ([#244](https://github.com/mairie360/Calendar_API/pull/244))
- test: MAIR-1002 Speed up the APIs' CI tests (Rust cache, one database per test, optimized dependencies) ([#248](https://github.com/mairie360/Calendar_API/pull/248))
- test: MAIR-1002 Speed up the APIs' CI tests (Rust cache, one database per test, optimized dependencies) ([#247](https://github.com/mairie360/Calendar_API/pull/247))
- chore(deps): update rust crate mairie360_api_lib to v4.4.0 (#225) ([#225](https://github.com/mairie360/Calendar_API/pull/225))
- test: MAIR-247 Translate the APIs' OpenAPI descriptions and messages into English (#242) ([#242](https://github.com/mairie360/Calendar_API/pull/242))
- chore(deps): MAIR-243 Bump quinn-udp to the patched 0.5.16 (RUSTSEC-2026-0538) (#239) ([#239](https://github.com/mairie360/Calendar_API/pull/239))
- fix: MAIR-247 Translate the APIs' OpenAPI descriptions and messages into English (#237) ([#237](https://github.com/mairie360/Calendar_API/pull/237))
- docs: MAIR-245 Rewrite the APIs' README and API.md (#227) ([#227](https://github.com/mairie360/Calendar_API/pull/227))
- chore: MAIR-243 Align the test Database version on the 3.4.0 release (#234) ([#234](https://github.com/mairie360/Calendar_API/pull/234))

## Différence staging → prod (en staging, pas encore en prod)

_Rien._

> Résumé Claude indisponible pour ce passage : listes brutes des commits.

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-11-17 14:47 UTC | dev | — | `ff6f17d` | [run](https://github.com/mairie360/Calendar_API/actions/runs/44104070671) |
| 2026-11-17 11:47 UTC | dev | — | `f5e8c7c` | [run](https://github.com/mairie360/Calendar_API/actions/runs/44075420740) |
| 2026-11-17 01:29 UTC | dev | — | `f572f65` | [run](https://github.com/mairie360/Calendar_API/actions/runs/43994217357) |
| 2026-11-17 00:10 UTC | dev | — | `c382a1c` | [run](https://github.com/mairie360/Calendar_API/actions/runs/43981548862) |
| 2026-11-16 23:10 UTC | dev | — | `30ce646` | [run](https://github.com/mairie360/Calendar_API/actions/runs/43974645500) |
| 2026-11-16 17:42 UTC | dev | — | `0049b2c` | [run](https://github.com/mairie360/Calendar_API/actions/runs/43919075281) |
| 2026-11-16 16:50 UTC | dev | — | `fa75131` | [run](https://github.com/mairie360/Calendar_API/actions/runs/43909704551) |
| 2026-11-16 16:42 UTC | dev | — | `2fc2ea5` | [run](https://github.com/mairie360/Calendar_API/actions/runs/43908285695) |
| 2026-11-16 16:39 UTC | dev | — | `a90182e` | [run](https://github.com/mairie360/Calendar_API/actions/runs/43905364158) |
| 2026-11-12 23:53 UTC | prod | v1.3.0 | `89649f7` | [run](https://github.com/mairie360/Calendar_API/actions/runs/43330545485) |
| 2026-11-12 18:51 UTC | staging | — | `89649f7` | [run](https://github.com/mairie360/Calendar_API/actions/runs/43282738519) |
| 2026-11-12 12:58 UTC | dev | — | `89649f7` | [run](https://github.com/mairie360/Calendar_API/actions/runs/43220669726) |
| 2026-11-12 11:52 UTC | dev | — | `f46efcf` | [run](https://github.com/mairie360/Calendar_API/actions/runs/43211612978) |
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
