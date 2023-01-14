# Copyright 2023 Hardik Phalet
echo "Moisturiser 0.1.0"

# making directory for our installation
mkdir -p "$HOME/.moisture/templates"

# defining abort function to, well, abort
abort() {
    printf "%s\n" "$@" >&2
    exit 1
}

# checking OS
OS="$(uname)"
if [[ "${OS}" != "Darwin" ]]
then
    abort "Moisturiser is only supported on macOS"
fi

# checking architecture
Arch="$(uname -m)"
if [[ "${Arch}" != "arm64" ]]
then
    abort "Moisturiser is only supported on arm64 (>M1 Macs)"
fi

# downloading the binary
(cd $HOME/.moisture && curl -LO https://github.com/hardikphalet/moisturiser/releases/download/alpha-0.2/moisturiser && chmod +x moisturiser)

# downloading templates
( cd $HOME/.moisture/templates && curl -O https://raw.githubusercontent.com/hardikphalet/moisturiser/master/templates/Controller.tjava )
( cd $HOME/.moisture/templates && curl -O https://raw.githubusercontent.com/hardikphalet/moisturiser/master/templates/Mapper.tjava )
( cd $HOME/.moisture/templates && curl -O https://raw.githubusercontent.com/hardikphalet/moisturiser/master/templates/Repository.tjava )
( cd $HOME/.moisture/templates && curl -O https://raw.githubusercontent.com/hardikphalet/moisturiser/master/templates/Response.tjava )
( cd $HOME/.moisture/templates && curl -O https://raw.githubusercontent.com/hardikphalet/moisturiser/master/templates/Service.tjava )

# downloading config file
( cd $HOME/.moisture && curl -O https://raw.githubusercontent.com/hardikphalet/moisturiser/master/config.json )

# replacing the location of templates in config
template_location="$HOME/.moisture/templates"
(cd $HOME/.moisture && sed -i .bak "s|\${TEMPLATE}|${template_location}|g" config.json)

# config location
echo export moist_config="$HOME/.moisture/config.json" >> ~/.zshrc

# adding path to zshrc
echo export PATH="\"\$PATH:$HOME/.moisture\"" >> ~/.zshrc
