project('asdcontrol-gnome', 'cpp',
  version: '0.1',
  default_options: ['cpp_std=c++17']
)

gtkmm_dep = dependency('gtkmm-4.0')

mode = get_option('mode')

if mode == 'dev'
  message('Development mode enabled')
elif mode == 'prod'
  message('Production mode enabled')
else
  error('Invalid mode. Use "dev" or "prod".')
endif

executable('asdcontrol-gnome',
  'src/main.cpp',
  dependencies: gtkmm_dep,
  install: true
)

install_data('scripts/build.sh', install_dir: get_option('prefix') / 'bin')
install_data('scripts/dev.sh', install_dir: get_option('prefix') / 'bin')
install_data('scripts/prod.sh', install_dir: get_option('prefix') / 'bin')

