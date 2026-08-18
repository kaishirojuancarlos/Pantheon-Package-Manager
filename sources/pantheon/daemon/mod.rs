use hive_library::hive::control::escalate_with_root_access;

pub fn launch_daemon()
{
	escalate_with_root_access()
		.unwrap();
	todo!("not yet implemented");
}
