default:
	rustc hello_gen.rs -Z debug-info

clean:
	rm -f hello_gen
