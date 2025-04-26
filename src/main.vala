using Gtk;

int main(string[] args) {
    var app = new Gtk.Application("com.example.asdcontrol-gnome", ApplicationFlags.FLAGS_NONE);

    app.activate.connect(() => {
        var window = new ApplicationWindow(app);
        window.title = "asdcontrol-gnome";
        window.set_default_size(600, 300); // Updated to use set_default_size()
        window.show();
    });

    return app.run(args);
}
