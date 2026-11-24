# Brain panel

The panel shows the possessed body's brain: inputs on the left, hidden neurons, outputs with gauges on the right, the strongest path as a sentence, and who drives ("The brain drives."). At 390 wide it is a sheet at the foot.

## Sub-features

- `brain-open`: the `Brain` button or B opens it, the close button or B closes it.
- `brain-live`: the gauges and the path sentence follow the body tick by tick.
- `brain-changes`: "Wiring changed since you first looked: n".
- `brain-states`: no body taken, a body that died, a brain that could not be read (`.brain-state`).

## How to get to it (user POV)

- Take a body, then press B or choose `Brain`.
- Choose `Brain` in Watch: the panel says to take a body first.

## Driving it with tidepool-verify

- **Open in Play.** `strip <url> $E/brain --play --act .claude/skills/verify-tidepool/acts/brain.mjs`. Each frame shows the region `Brain` with its gauges. The `Brain` button has `aria-pressed="true"`.
- **Phone.** Add `--size 390x844`. The sheet must not cover the possessed body or the top buttons, and the hint line must stay above the sheet.

## Gotchas

- Output names next to close gauge rows overlapped before `d7f0a1ac`. Check them on a brain with many outputs.
- Opening the panel makes each draw slower in the software GPU. The strip still lands on exact ticks.
