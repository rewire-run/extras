<!--
Title this PR with Conventional Commits, e.g. `feat: add QoS columns to ROS2TopicInfo`.
Keep the PR focused — one logical change per PR.
-->

## Summary

<!-- What does this PR change and why? Link the motivation, not just the diff. -->

## Changes

<!-- Bullet the notable changes. Call out anything reviewers should look at first. -->

-

## Related

<!-- GitHub issue or ADR. e.g. Closes #123 -->

-

## Checklist

- [ ] PR title follows [Conventional Commits](https://www.conventionalcommits.org/)
- [ ] `pixi run sanity` passes locally (check + fmt + lint + test)
- [ ] `cargo clippy --all-features --all-targets -- -D warnings` is clean
- [ ] `cargo test --all-features` passes
- [ ] Generated gRPC code is regenerated with `pixi run generate-proto` if a `.proto` changed
- [ ] Public items have `///` docs

## Breaking changes

<!-- Write "None" if none. Call out renamed or removed archetype fields, changed component descriptors, Rerun version bumps, or `.proto` changes. The bridge and viewer must move together on any of these. -->
