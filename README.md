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
