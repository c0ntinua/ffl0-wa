# ffl0-wa

This is another version of ffl0 for terminals (especially webassembly runtimes) with a slight adjustment of the update rule. Instead of using a read buffer and a write buffer, here a single buffer is used, and this works because not all cells are updated simultaneously. The primary motivation was not to save memory but to eliminate a flickering effect that's present on terminal versions of ffl0 but not in true graphical versions. The change seems to improve the situation significantly. I've also added a parameter, the number of updates, which can be chosen to go along well with the host computer and the other parameters. I should note that I sometimes zoom-out in iTerm2 to make the unicode blocks very small. This makes it possible to have something like 100 rows and 200 columns (or more even.) One thing still missing from the 'web assembly in the terminal' versions is input. So the user can only watch the random fireworks, eventually using control-C to stop the program. I'd be grateful for any advice on this issue.
