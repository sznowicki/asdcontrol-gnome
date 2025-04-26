using Gtk;
using GLib;

errordomain CheckError {
    COMMAND_NOT_FOUND,
}

namespace Checks {

    public void checkEnv(ApplicationWindow parent) {
        try {
            var process = new Subprocess(
                SubprocessFlags.STDOUT_PIPE,
                "which",
                "aasdcontrol"
            );
            process.wait();
            if (process.get_exit_status() != 0) {
                throw new CheckError.COMMAND_NOT_FOUND("no asdcontrol");
            }
            stdout.printf("Environment check completed.\n");
        } catch (Error e) {
            var dialog = new AlertDialog("No ASDControl found");
            dialog.set_message("Please install ASDControl to use this application.");
            var result = dialog.choose(parent);

        }
    }

}
