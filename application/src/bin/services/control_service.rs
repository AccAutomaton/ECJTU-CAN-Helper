use std::{ffi::OsString, path::PathBuf};

use windows_service::{
    service::{
        ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceState,
        ServiceType,
    },
    service_manager::{ServiceManager, ServiceManagerAccess},
};

pub fn create_and_register_service() -> Result<(), windows_service::Error> {
    let service_name = OsString::from("Ecjtu_CAN_Helper");
    let service_display_name = OsString::from("华东交通大学校园网助手");
    let service_path = OsString::from(".\\service.exe");
    let settings_path = OsString::from(".\\settings.json");
    let launch_arguments = vec![settings_path];

    let manager =
        ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CREATE_SERVICE)?;

    if let Ok(service) = manager.open_service(&service_name, ServiceAccess::QUERY_STATUS) {
        let status = service.query_status()?;
        match status.current_state {
            ServiceState::Running => {
                service.stop()?;
                service.start(&launch_arguments)?
            }
            _ => service.start(&launch_arguments)?,
        }
    } else {
        let service_info = ServiceInfo {
            name: service_name,
            display_name: service_display_name,
            service_type: ServiceType::OWN_PROCESS,
            start_type: ServiceStartType::AutoStart,
            error_control: ServiceErrorControl::Normal,
            executable_path: PathBuf::from(service_path),
            launch_arguments: launch_arguments,
            dependencies: vec![],
            account_name: None,
            account_password: None,
        };
        manager.create_service(&service_info, ServiceAccess::START)?;
    }

    Ok(())
}
