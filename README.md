# shall
Some non-working stuff I didn't even think of a proper name for. I got a few ideas but nothing concrete.

The idea would be to have some shell / text editor ( aka bash IDE but not for bash ) with blackjack and improved color support.

I'll start with just implementing a simple text editor with rust with termion, then mb add better colors, then design new scripting language (I might steal many things from fish), write interpreter for that, then make the text editor into IDE and then into a shell.
After all that do something about programs like vim or similar that steal the entire terminal window - mb detect that there is a window manager and use that? No idea really.

Rationale:
1) bash as scripting language kinda sucks
2) bash as shell also kinda sucks - the editing of commands is not easy
3) the colors - one can write using colors in bash, but one has to look up escape codes; also, those constantly get lost in pipes between tools which carefully check if they are printing to tty or not
4) I believe turning shell into IDE for corresponding scripting lang would be the way to go.

I might try to impletent this as plugin for neovim instead.


Regarding the *editor* part:
1) use the client server architecture - this way if one opens a completely different terminal one can still attach to the same session (without need for something like tmux)
2) for a single window use only file buffers, no tabs (with the ability to quickly switch between those via something like ctrl-P) - there are already too many tabs - tmux got tabs and windows, i3 got the same, there is just no need for tabs. Also, one would want to use WM windows/tabs instead of internal ones most of the time, so why have both?

Regarding REPL part:
since commands might capture input/screen (f.e. if we try to open something like vim) the default behavior would be to run commands in the background (like `vim &` in bash), and maybe if those finish then put the output in the current terminal? One thing unclear is how to open/close the applications then. Rn in bash I can run `vim &`, then I can `fg` to open it and `ctrl-z` to hide it again. Hence an idea - use `ctrl-z` as the key to switch buffers.
So in `shall` one would run `vim`, press `ctrl-z` to switch to vim window, and after being done press `ctrl-z` again to switch to any other buffer.

Things it should be able to do:

1. run registered executables and capture their output (in bash it's those in $PATH)
2. behave as REPL of some language (handle `if/else/while/for/let/lambda/exec` etc)
3. do something about current working directory (minimal - implement `cd`, then mb `pushd` and other things for directory stack)
4. provide QoL and customizaton for the `shall` itself - f.e. `alias` and adjusting `powerline` - not sure if this should be done inside or only as part of some config (if config is written in the same lang, then one can just run through it at the start); also `open/e` for `xdg-open` or something. 
5. piping, idealy also from processes to local functions
6. autocompletion of some sort?
7. `source/include`
8. signal handling - `emit/trap`
9. jobs & switching between those (see concept above)

I think this covers most of the features I want. Whole lot of those can be stolen from fish-shell, with the biggest changes being in items 1 and 9, maybe 3 and 5 (those require some thinking). The biggest change would be in handling the overall work-process, (should be somewhat similar to that of jupyter).
