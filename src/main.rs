use win32_notif::{
  notification::{
    actions::{action::ActivationType, input::InputType, Input},
    header::{Header, HeaderActivationType},
    visual::progress::Progress,
    RawXML,
  },
  refvar, string, NotificationBuilder, ToastsNotifier,
};

fn main() {
  let notifier = ToastsNotifier::new("Microsoft.Windows.Explorer").unwrap();

  let notif = NotificationBuilder::new()
    .visual(Progress::new(
      None,
      string!("Downloading..."),
      string!("0.30"),
      None,
    ))
    .build(1, &notifier, "a", "ahq")
    .unwrap();

  notif.show();

  loop {}
}
