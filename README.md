## Git Helper CLI

The idea here is to create a CLI from which user can search for repositories based on filter conditions and delete them if they want to.

# Plans
[ ] Connect to github via device flow
[ ] Store token in os specific folders for app data __( preferrably encrypted )__
[ ] User should be able to select multipl repos and delete

# Filters
[ ] Empty - should list all repos ( paginated )
[ ] Repo name - based on used provided repo name/names __( variadic )__
[ ] File count - if github api supports file count __( range should also be supported )__
[ ] File names - based on the file name/names provided by the user.. list repos with files of given name
[ ] File names regex
[ ] File names union or intersection __( or / and )__

# Goals of exercise

Use less crates as possible, refer but don't use. build own.
