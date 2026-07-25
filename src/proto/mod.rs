/// Rewire gRPC service definitions (v1). Superseded by [`v3`]; kept published
/// for older binaries, no current rewire binary speaks it.
pub mod v1;

/// Rewire gRPC service definitions (v2). Superseded by [`v3`]; kept published
/// for older binaries, no current rewire binary speaks it.
pub mod v2;

/// Rewire gRPC service definitions (v3): `RelayService` and `BridgeService`.
/// The current protocol line — `ViewerService` is gone (viewers are pure relay
/// clients since rewire-viewer 0.6.0).
pub mod v3;
