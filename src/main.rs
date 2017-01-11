mod interpreter;
use interpreter::default_opcodes::DefaultOpcodes;

fn main() {
	let bytecode: Vec<u8> = vec![0x02, 0x00, 0x01, 0xF4, 0xFF, 0xFF, 0xFF, 0x01, 0x00, 0x05, 0xFF, 0x00, 0x01, 0x00, 0x05, 0xF5, 0x00, 0x02, 0x00, 0x01, 0xF9, 0xFF, 0xFF, 0xFF];

	let mut cleo_vm = interpreter::VirtualMachine::new();

	cleo_vm.set_default_opcodes();
	cleo_vm.append_script(bytecode);
	cleo_vm.run();
}
