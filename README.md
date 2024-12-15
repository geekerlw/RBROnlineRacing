# RBR Online Racing
Richard Burns Rally (RSF) realtime online racing support.

## Features
- Fully automated lobby workflow, you just need to select car, set it up and *wait*.
- Real-time leaderboard and progress bar in game.
- Ranking system with score record, 3 points if you finish race, -5 if you retired.
- Online Ghost Car support, but only one ghost per race, **and** if you previously set up any time on the current track (recorded any ghost).
- Races start every three minutes with randomised stage, weather.
- Voice announcer when join, leave, prepare etc.
- Some maps are excluded from pool such as mazes, bad roads, and too long maps.

## How to install
Download latest RBNHelper_xxx.zip release and unzip it into the root of your Richard Burns Rally folder.

## How to race with others
1. Open game normally and Login to RSF main menu.
2. You will see info about the server, such as how many players online and time duration to next race in top center of your screen.
3. Join race by Enter `Hotlap` or `Practice` anytime, **just stay at this menu page, dont start race manually**.
4. After the countdown ends, there will be additional 30 seconds to prepare, then you will be automatically launched into the race.
5. **Be patient, dont press anything.** Once all players loaded, server will pull the handbrakes for everybody at the same time, and start 5 second timer.
6. Drive carefully, you can see other player's progress on the screen left and an brief leaderboard on the top-left screen.
7. If you are first to the finish line, wait in results page, leaderboard will be updating.
8. Go back to `Hotlap` or `Practice` menu, and wait for the next race to begin.

## Custom configuration
You can Find an `RBNHelper.ini` file under `Plugins\RBNHelper` folder, edit it and *restart game*.
Configurations available for you to change:
- main plugin features.
- colors, font size of leaderboard and progress bar.
- positions of online leaderboard and progress bar on the screen.
- server you want to play on.
- announcer voice, volume and speed.

## How to uninstall
Only need to remove the rbnhelper.dll and `RBNHelper` folder from the game's `Plugins` folder.
Or just disable the plugin from RSF launcher.