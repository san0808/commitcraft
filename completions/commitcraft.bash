#!/bin/bash

_commitcraft() {
    local cur prev opts
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"

    # Main commands
    local commands="setup config list"
    
    # Options
    local opts="--provider --model --dry-run --review --force --verbose --include-files --show-command --legacy --yes --help --version"
    
    # Providers
    local providers="openai gemini anthropic"
    
    # Models
    local models="gpt-4o gpt-4o-mini gpt-4-turbo gpt-3.5-turbo gemini-1.5-pro-latest gemini-1.5-flash-latest gemini-1.0-pro claude-3-5-sonnet-20241022 claude-3-haiku-20240307 claude-3-opus-20240229 fast smart"

    case $prev in
        --provider|-p)
            COMPREPLY=($(compgen -W "$providers" -- "$cur"))
            return 0
            ;;
        --model|-m)
            COMPREPLY=($(compgen -W "$models" -- "$cur"))
            return 0
            ;;
    esac

    # Complete commands and options
    if [[ $cur == -* ]]; then
        COMPREPLY=($(compgen -W "$opts" -- "$cur"))
    else
        COMPREPLY=($(compgen -W "$commands" -- "$cur"))
    fi
}

complete -F _commitcraft commitcraft 