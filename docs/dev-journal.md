I have restructured the folder structure to better allign with a development sandboxing model Where each code responsible of being executed resides in /code 

I have implemented data aggregation with raw sql initially, it works fine, however using a seaorm query builder is better for code maintainability, especially to get errors early on any schema changes 

it looks like I have realized something which I have previously missed. I can't simply take the diffs of an aggregation of sdk usage and call it a day, since an app may have multiple payment sdk's, and may implement more sdk's without losing any of them. I need to be more spesific

This visualisation is concerned about how many apps may have changed their payment sdk, in favor of which competitor, so I will be counting under the following conditions instead:
   A-B-C-D A-B-C-D
if 0-0-0-0 0-0-0-0 Retention none 
if 1-0-0-0 0-0-0-0 A => none
if 1-0-0-0 1-1-0-0 There is no churn here. There is retention for A, not B: A did not lost anything, this app was not owned by the mystery "none"
if 1-0-1-0 1-1-0-0 C => B, Retention A. Here I will feel justified in saying that this app has migrated from C to B
if 0-0-0-0 1-1-0-0 none to =>A,B,C,D
if 0-0-0-0 1-0-0-0 none => A
if 1-1-1-0 0-0-0-1 A=>D,B=>D,C=>D 

It is also possible that an app, has just started to be tracked, such that previous data has so record of this app, in that case I will exclude that app from the calculation process alltogether, since no retention, or attrition data makes sense from a single historical data point.

It is possible that a new sdk has started to be tracked, now in order to stay consistent, I must exclude these from the calculation as well, since it will also show no insight to neither attrition or retention

for these cases, it makes sense to still visualise this where relevant, perhaps as a footnote or tooltip  since they are not readily appereant

Now that I reconsider it, none is an sdk which is not included in the initial filter, not that we don't know what they use, so a revised table:

   A-B-C-D A-B-C-D
if 0-0-0-0 0-0-0-0 no data, not to be included
if 1-0-0-0 0-0-0-0 A has lost this app, but not to "to another solution not covered in this matrix",
if 1-0-0-0 1-1-0-0 There is no churn here. There is retention for A, not B: A did not lost anything, this app was not owned by the mystery "none"
if 1-0-1-0 1-1-0-0 C => B, Retention A. Here I will feel justified in saying that this app has migrated from C to B
if 0-0-0-0 1-1-0-0 no churn, no retention, there is addition of A and B, There is no place for this in the matrix, but I will calculate this anyways since it is useful information 
if 0-0-0-0 1-0-0-0 Same as above
if 1-1-1-0 0-0-0-1 A=>D,B=>D,C=>D


This version makes more sense now, since my initial model was assuming that the app could have had an sdk we were unable to detect, which was the "none" in the image, however I  realize that I was wrong in that. None is simply data that the user chose not to display, perhaps the keyword "other" is better here.

I have found another solution on paper, I am going to needto create an intermidiate layer, and do my calculations exclusively from this layer.
Once this intermidiate layer is calculated, then I can use this as sort of a cache, and use it as a way to take back the potential memory consumption, sinceI can  create this intermidiate layer using only the app-sdk per app id

(data 1 x data 2) => get sdk usage of app 1 =>intermidiate layer, where it defines events,   app 1 left  A for B => user requests => data is fetched from the intermidiate layer. I don't think this is the best solution, but I feel confident that it will function and reasonably scale. This is under the assumption that a single app will not implement millions or billions of sdk changes between 2 data points

Getting example apps for a given matrix is more work than I initialy thought, for the sake of time, I am going to have to concede on inmplementing it




## todo & ideation

[x] using seaorm genereate the types
    - seaorm generation resulted in a broken field, where app id was some, removing it fixed a not implemented trait error, should explore this later
    - entity is now available as a workspace lib
    * in app_sdk table, there is no installed is NULL ,so I am going to relieve it from the option 
    * it looks like seaorm generation may have not been perfect, I should have double checked, might need to revise

[*] Implement differential engine
    - This is basically calculating and rendering diffs based on 2 datasets
    
[*] data should be accesible via the data folder, in production data would ideally be stored in an immutable data store,
    * I could add data upload for convinience
    * As a shortcut for assignment purpouses, I can make the db file permissions read only rather than implement a object store
[*] genereate filters for sdks
    * implement a simple debounce filter field to get a list of fitting sdk's 
        * I could consider alternate solutions here to accomodate high quantities of data
[*] handle rendering
    * howering over the fields should highlight rows and columns
    * howers could also show some app names and or logos
    - I chose to use rust for webassembly for the frontend since it should fit nicely with the rest of the codebase
    - Initially I chose webassembly, but since then I have reazlied that there isn't a big need to utizile much webassembly, I will still keep it in the project, since it is not detrimental, and it may acome in handy, if renders of bigger matrixes are needed, or for some reason there is data to be processed locally instead of the server.
    *By using htmx , I can respond in html snippets, while using the rust type checker in generating those snippets. Otherwise I would have had to create an openapi spec, then generate a frontend sdk using that spec, for a project this size, htmx is a solid alternative

