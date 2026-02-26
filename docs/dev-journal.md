I have restructured the folder structure to better allign with a development sandboxing model Where each code responsible of being executed resides in /code 



## todo & ideation

[x] using seaorm genereate the types
    - seaorm generation resulted in a broken field, where app id was some, removing it fixed a not implemented trait error, should explore this later
    - entity is now available as a workspace lib
[ ] data should be accesible via the data folder, in production data would ideally be stored in an immutable data store,
    * I could add data upload for convinience
    * As a shortcut for assignment purpouses, I can make the db file permissions read only rather than implement a object store
[ ] genereate filters for sdks
    * implement a simple debounce filter field to get a list of fitting sdk's 
[ ] handle rendering
    * howering over the fields should highlight rows and columns
    * howers could also show some app names and or logos

[ ]optimize
