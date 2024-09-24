# oxdb-engine

> ## oxdb-engine - in Dev


# Architectural Design [ Oxdb ] :

## compressed design :

```py

# db-level struct: 
struct = db:doc:record
# db-level uri : 
uri = db:doc:rid.data
uri = db:doc:uid.data


#---------------------------------------------
# struct expansion :
db  = phy folder    #- entire database 
doc = phy file      #- individual unit of database 
record = phy data   #- a cantionus block of bin data in a disk
# uri    expansion :
uri = res locator   # used as index to locate data
rid = genratd locId #- generated on demand from doc rec by scan and cached
uid = hash key      #- in memory maped to rid after scan
data = Data     #- simplay a data we are seeking for 

#---------------------------------------------
rid = ( block_index , startpos_blocklevel , schemaid_opt , named_opt ) generated based on 
uid = uuid , keystr , named 

# low-level data unit
data = type t : any = [ r , p , x , n , i , f , s , l , v , m , c ]

# high-level data unit ( record , row , rec )
record = # a single unit of doc all datas are stored as and within rec


#---------------------------------------------
oxdbin = ox data binary # an ox standard bin data representation  
# for efficient storage and scaning of bin data and convertion to usable data 
# all binary representation of oxdbin contain 3 components 
# standard unit  Identifier | Size | Data | 
(bytes) len = [ I = (2) , S = (4,8) , D = (n =[0-"2^32"-"2^64"]) ]

# record :
oxdbin  = Identifier   | keystr n bytes  |Size| Data  | 
record  =   r : record | keys_str+"\x00" |  8 | Data  |
planid  =   p : schema |               i int or s str |

# uri :
oxdbin  = Identifier   |Size| Data  | 
uri     =   x : uri    | 4  | s str |
del     =   n : delbyte| 4  | # may packed with "\x00" but ignored

# nums : no size rep direct data 
oxdbin  = Identifier    | Data  | 
int     =   i : integer | max = 8 bit 
float   =   f : float   | max = 8 bit 

# strings :
oxdbin  = Identifier    |Size| Data  | 
str     =   s : string  | 4  |max(n) = "2^32" or "4gb"
large   =   l : large_s | 8  |max(n) = "2^64"bytes (theoretically)

# collections :
oxdbin  = Identifier    |Size| Data  | 
vec     =   v : vector  | 4  | any   |
map     =   m : hashmap | 4  | s str , t any |

#custom :
oxdbin  = Identifier    | cID | cData  | 
cstm    =   c : custom  | str | t any  | #Custom type 


type t : any = [ r , p , x , n , i , f , s , l , v , m , c]
#---------------------------------------------

# data return structure
uri = db:doc:rid.data 
data = record
# return out
record = {
    rid : val ,
    uid : val ,
    Data : val ,
    size : val ,
    blockindex : val ,
    blockpos : val ,
    schema : planid ,
}
```


