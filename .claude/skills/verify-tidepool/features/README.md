# Tidepool feature map

One file per feature the player can reach. Each file says what the feature is, how a player gets to it, how to drive it with `tidepool-verify.mjs`, and what end state proves it works. Keep it true: when a change adds an entry point, add it to the feature's file.

| Feature | File | Entry points |
|---|---|---|
| Watch the world | [watch.md](watch.md) | the page with no query, `?world=`, `?founders`, `?debug` |
| Play a body | [play.md](play.md) | a click on a body, an heir on the death overlay |
| Brain panel | [brain.md](brain.md) | the `Brain` button, the `B` key, its close button |
| Field guide | [guide.md](guide.md) | the `Guide` button, the `G` key, `?` marks that open notes |

Not mapped yet: the death overlay (heirs, "Keep watching"), the bud and mate cards beyond what `play.md` shows, `?bench`, and the walkthrough beats (`?walk=reset`).
