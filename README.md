# oxdb-engine

> ## oxdb-engine - in Dev



# Architectural Design [ Oxdb ] :

## compressed design :

```py

# db-level struct: 
struct = db:doc:record
# db-level uri : 
uri = db:doc:rid.data
uri = db:doc:iid.data
uri = db:doc:uid.data

rid  = ( pid , sid ) -> recpos
iid -> ( pid , sid ) -> recpos
uid -> ( pid , sid ) -> recpos
uid ->      iid      -> recpos

recpos -> rec.data

#---------------------------------------------
# struct expansion :
db  = phy folder    #- entire database physical folder 
doc = phy file      #- .oxd individual unit of database 
record = phy data   #- a cantionus block of bin data in a disk
index = phy data    #- .oxdi index file that stored all indexes 

# uri    expansion :
uri = res locator   # used as index to locate data
rid = locationID    #- generated on demand from doc rec by scan and cached
iid = indexID       #- numerical index for records (sorted sequencely generated)
uid = hashkeyID     #- (uuid,keystr,named) in memory maped to rid after scan 
pid = pageIndex     #- pageindex
sid = slotIndex     #- slotIndex
recpos =  pagepos   #- record pos with in the pages
data = Data         #- simplay a data we are seeking for 
#---------------------------------------------


# low-level data unit
data = type t : any = [ r , x , z , n , i , f , s , l , v , m , c ]
# high-level data unit ( record , row , rec )
record = # a single unit of doc all datas are stored as and within rec


#---------------------------------------------
oxdbin = ox data binary # an ox standard bin data representation  
# for efficient storage and scaning of bin data and convertion to usable data 
# all binary representation of oxdbin contain 3 components 
# standard unit  Identifier | Size | Data | 
(bytes) len = [ I = (2) , S = (4,8) , D = (n =[0-"2^32"-"2^64"]) ]

# record :
oxdbin  = Identifier   | iid |keystr n bytes  |Size| Data  | 
record  =   r : record | int |keys_str+"\x00" |  8 | Data  |


# uri :
oxdbin  = Identifier   | Data           | 
uri     =   x : uri    | i int or s str |
planid  =   z : schema | i int or s str |
del     =   n : delbyte| 4  |  "\x0"    | # may packed with "\x00" but ignored

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

# data return output structure
uri = db:doc:rid.data 
data = record
# return output
record = {
    rid : val ,
    iid : val ,
    uid : val ,
    size : val ,
    Data : val ,
    zid : planid or schema ,
    pid : pageindex ,
    sid : slotindex ,
}
#---------------------------------------------
# doc storage :
blocks = 4kib #default dependence on system hardware
pages  = n * blocks  # n = 16 default
pages -> slotted pages [ slot-array , [records]]
pid -> sid -> recpos -> data

# index storage :
rule-1 :
# sorted continues sequence of block
iid : 8b -> pid : 8b , sid : 4b
# o(1) look up with iidpos = iid * 20b
rule-2 :
# Btree index
iid -> pid , sid
# o(log(n)) look up binary search
#---------------------------------------------
# Transaction :
#each opreation is Transc by default
start edit
multiple operations 
end commit
# flushed after commit to disk
```
