//! # DBus interface proxy for: `org.freedesktop.PackageKit.Offline`
//!
//! This code was generated by `zbus-xmlgen` `4.0.0` from DBus introspection data.
//! Source: `org.freedesktop.PackageKit.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus2.github.io/zbus/client.html)
//! section of the zbus documentation.
//!

use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.freedesktop.PackageKit.Offline",
    assume_defaults = true
)]
trait Offline {
    /// Cancel method
    fn cancel(&self) -> zbus::Result<()>;

    /// ClearResults method
    fn clear_results(&self) -> zbus::Result<()>;

    /// GetPrepared method
    fn get_prepared(&self) -> zbus::Result<Vec<String>>;

    /// Trigger method
    fn trigger(&self, action: &str) -> zbus::Result<()>;

    /// TriggerUpgrade method
    fn trigger_upgrade(&self, action: &str) -> zbus::Result<()>;

    /// PreparedUpgrade property
    #[dbus_proxy(property)]
    fn prepared_upgrade(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// TriggerAction property
    #[dbus_proxy(property)]
    fn trigger_action(&self) -> zbus::Result<String>;

    /// UpdatePrepared property
    #[dbus_proxy(property)]
    fn update_prepared(&self) -> zbus::Result<bool>;

    /// UpdateTriggered property
    #[dbus_proxy(property)]
    fn update_triggered(&self) -> zbus::Result<bool>;

    /// UpgradePrepared property
    #[dbus_proxy(property)]
    fn upgrade_prepared(&self) -> zbus::Result<bool>;

    /// UpgradeTriggered property
    #[dbus_proxy(property)]
    fn upgrade_triggered(&self) -> zbus::Result<bool>;
}