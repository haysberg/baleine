# if running bash
if [ -n "$BASH_VERSION" ]; then
    # include .bashrc if it exists
    if [ -f "$HOME/.bashrc" ]; then
        . "$HOME/.bashrc"
    fi
fi

# set PATH so it includes user's private bin if it exists
if [ -d "$HOME/bin" ] ; then
    PATH="$HOME/bin:$PATH"
fi

# set PATH so it includes user's private bin if it exists
if [ -d "$HOME/.local/bin" ] ; then
    PATH="$HOME/.local/bin:$PATH"
fi

# launch docker bash if logging in through SSH
if [[ -n $SSH_CONNECTION ]] ; then
    #extracts the name of the first container on the list. There should be only one per machine anyway.
    container_name=$(docker ps | sed '2q;d' | cut -d" " -f32)
    #if there is no containers running currently
    if [ "$container_name" == "" ]; then
        echo "There is no container currently running on this machine."
        docker container ls -a
        exit
    fi
    docker exec -it $container_name bash
    exit
fi