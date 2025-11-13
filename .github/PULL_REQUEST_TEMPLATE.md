# Pull Request checklist and template

Use this template to create clear, reviewable pull requests. Keep the Summary short (1–2 lines) and add details in the sections below.

---

## Type of change (select or check all that apply)

- [ ] Bugfix — fixes a bug without changing public API
- [ ] Feature — adds new functionality (may change public API)
- [ ] Docs — documentation only changes
- [ ] Chore — tooling / CI / formatting / non-functional change

## Summary (1–2 lines)

Concise description of what changed and why. If this implements a proposal, reference it.

## What I changed (short list)

- Brief file/behavior list (what's touched and why). Example: `engage-ux-core/src/lib.rs: add X`.

## Motivation and context

Why is this change needed? Who benefits? Link any relevant design notes or issue discussions.

## Related issues / PRs

- Fixes / relates to: #<issue-number> (use `Fixes #123` to auto-close when merged)

## Tests performed

Describe how you tested the change and how reviewers can reproduce locally. Include exact commands and any feature flags.

Example:

```bash
# run unit tests for the workspace (adjust as needed)
cargo test -p engage-ux-core

# run an example where applicable
cargo run -p engage-ux -- --example basic
```

Environment: e.g. Ubuntu 22.04 aarch64, Rust 1.70.0 (do not include secrets)

If this is a code change, include at least one unit test or an integration test demonstrating the behavior.

## Backwards compatibility / migration notes

If this change affects public APIs, document migration steps. Per project policy, add a one-line entry to `CHANGELOG.md` (top of file) for user-visible changes, using `YYYY-MM-DD — <author>: <short description>` format. If the change is breaking, mark it `(BREAKING CHANGE)`.

## Checklist for reviewers (please check before marking ready)

- [ ] I ran the relevant tests locally and they pass
- [ ] I added/updated unit tests for my change (if applicable)
- [ ] I updated documentation where needed (docs or inline comments)
- [ ] If this changes public behavior, I added a `CHANGELOG.md` entry at the top of the file
- [ ] I considered security and performance implications

---

## Notes for maintainers (optional)

Any extra details useful for the maintainer (release considerations, rollout plan, or manual steps).

