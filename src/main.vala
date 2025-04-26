using Gtk;
using Checks; // Import the new Checks namespace

int main(string[] args) {
    var app = new Gtk.Application("com.sznowicki.asdcontrol-gnome", ApplicationFlags.DEFAULT_FLAGS);


    app.activate.connect(() => {
        var window = new ApplicationWindow(app);
        window.title = "asdcontrol GNOME GUI";
        window.set_default_size(600, 300); // Updated to use set_default_size()
        window.present(); // Updated to use present() instead of show()
        checkEnv(window); // Call the checkEnv function

    });

    return app.run(args);
}

