cargo build -p {{ cookiecutter.project_name }} --release
rm -rf ~/.lv2/{{ cookiecutter.project_name }}.lv2
cp ./target/release/lib{{ cookiecutter.project_name }}.so ./{{ cookiecutter.project_name }}.lv2/lib{{ cookiecutter.project_name }}.so
cp -r ./{{ cookiecutter.project_name }}.lv2/ ~/.lv2/{{ cookiecutter.project_name }}.lv2
