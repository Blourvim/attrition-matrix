I have restructured the folder structure to better allign with a development sandboxing model Where each code responsible of being executed resides in /code 

I have implemented data aggregation with raw sql initially, it works fine, however using a seaorm query builder is better for code maintainability, especially to get errors early on any schema changes 



## todo & ideation

[x] using seaorm genereate the types
    - seaorm generation resulted in a broken field, where app id was some, removing it fixed a not implemented trait error, should explore this later
    - entity is now available as a workspace lib

[ ] Implement differential engine
    - This is basically calculating and rendering diffs based on 2 datasets
    
[ ] data should be accesible via the data folder, in production data would ideally be stored in an immutable data store,
    * I could add data upload for convinience
    * As a shortcut for assignment purpouses, I can make the db file permissions read only rather than implement a object store
[ ] genereate filters for sdks
    * implement a simple debounce filter field to get a list of fitting sdk's 
        * I could consider alternate solutions here to accomodate high quantities of data
[ ] handle rendering
    * howering over the fields should highlight rows and columns
    * howers could also show some app names and or logos
    - I chose to use rust for webassembly for the frontend since it should fit nicely with the rest of the codebase

[ ]optimize

[ ] consider fault tolerance
    * Could certain amount of inaccuracy be tolerated ?
    * I should recheck my code, and handle faults more gracefully
