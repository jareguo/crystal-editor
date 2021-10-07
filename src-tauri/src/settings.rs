// Global settings that's synced across all windows.
#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}