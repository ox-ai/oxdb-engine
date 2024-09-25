
# Architectural Design [ Oxdb ] :

## compressed design :

```py
#---------------------------------------------
# db-level struct:
struct = db:doc:record
# db-level uri :
uri = db:doc:rid.data
uri = db:doc:iid.data
uri = db:doc:uid.data
# full uri flow structure
uri = db:doc:rid.pid.sid.recpos.rec.data

rid = ( pid , sid )
iid  -> rid
uid  -> rid

#---------------------------------------------
# struct expansion :
db  = phy folder    #- entire database physical folder
doc = phy file      #- .oxd individual unit of database
record = phy data   #- a cantionus block of bin data in a disk
index = phy data    #- .oxdi index file that stored all indexes
cache = cashe file  #- .oxdc pressisted inmemory data

#---------------------------------------------
# uri    expansion :
uri = res locator   #- used as db level index to locate data
rid = locationID    #- generated on demand from doc rec by scan and cached
iid = indexID       #- numerical index for records (sorted sequencely generated)
uid = hashkeyID     #- (uuid,keystr,named) in memory maped to rid after scan
pid = pageIndex     #- pageindex slotted pages
sid = slotIndex     #- slotIndex slotarray point to recpos
recpos =  pagepos   #- record pos with in the pages
data = Data         #- simplay a data we are seeking for

#---------------------------------------------
# data units :

# high-level data unit :  ( record , rec , row )
record = # a single unit of (doc and db) any data
# all datas are stored as and within rec
# low-level data unit  :
data = type t : any = [ r , x , n , i , f , s , l , v , m , c ]

#---------------------------------------------
# oxdbin :
oxdbin = ox data binary # an ox standard bin data representation
# for efficient storage and scaning of bin data and convertion to usable data
# all binary representation of oxdbin contain 2 or 3 components
# standard unit  Identifier | Size | Data |
(bytes) len = [ I = (2) , S = (4,8) , D = (n =[0-"2^32"-"2^64"]) ]

# record :
oxdbin  = Identifier    | iid |Size|      uid       | Data  |
record  =   r : record  | int |  8 |keys_str+"\x00" | Data  |


# uri :
oxdbin  = Identifier    |    Data        |
uri     =   x : uri     | i int or s str |
del     =   n : delbyte | 4  |  "\x0"    | # may packed with "\x00" but ignored

# nums :
oxdbin  = Identifier    |    Data     | #no size rep direct data
int     =   i : integer | max = 8 bit |
float   =   f : float   | max = 8 bit |

# strings :
oxdbin  = Identifier    | Size | Data          |
str     =   s : string  |  4   |max(n) = "2^32" or "4gb"
large   =   l : large_s |  8   |max(n) = "2^64"bytes (theoretically)

# collections :
oxdbin  = Identifier    | Size | Data          |
vec     =   v : vector  |  4   | t any         |
map     =   m : hashmap |  4   | s str , t any |

# custom :
oxdbin  = Identifier    | cty  | cData  |
cstm    =   c : custom  | str  | t any  | #Custom type

# uri units :
sid     = xsid:4b+2b+2b+2b+....+n*2b+"\x0"     # stream of bytes ends with "\x00"
pid     = xpid:4b+4b+4b+4b+....+n*4b+"\x0"     # used to store lookups
zid     = xzid:4b+4b+4b+4b+....+n*4b+"\x0"     #- schema id or plan
iid     = xiid:4b+4b+4b+4b+....+n*4b+"\x0"     #- unique num id of rec
rid     = xrid:4b+10b+10b+.....+n*10b+"\x0"    #- (pid+sid)
uid     = xuid:str+str+........+n*str+"\x0"    #- (uuid or unique keystr named)


#---------------------------------------------
# db map :
# map rep encode doc to oxdbin and decode it to map 
# physical data rep
db : {
    doc : {
        pagecount : pc 8b = dynamic True ,
        pagesize  : psz 3b (n 1b * blocksz 2b) = 16 * 4Kb = 64kb True ,
        zid : 4b+..+n*4b+"\x0" Optional , # meaning only valid inside db scope
        pid : 4b = dynamic False : {
            sid : 4b+..+n*2b+"\x0" True :  { 
                rec : iid 4b : size 8b : uid nb+"\x0" : = dynamic True : {
                    data : type t : any = [ r , x , n , i , f , s , l , v , m , c ]
                }
            }
        }
    }
}
# json out put rep
db : {
    doc : {
        pagecount : 10
        pagesize : 64kb
        zid : [11910,19189]
        1 : {
            2 : 101 : {
                "key1" : data
            }
            3 : 256 : {
                "key2" : data
            }
             
        }
    }
}
# uri for above sample
uri = db:doc:1:2:101:"key1":data.val
uri = db:doc:pid:sid:iid:uid:data.val
uri = db:doc:rid:iid:uid:data.val
uri = db:doc:rid:rec


#---------------------------------------------
# doc storage :
blocks = 4kib #default dependence on system hardware
pages  = n * blocks  # n = 16 default
pages -> slotted pages [ slot-array -- , --> [records]]
pid -> sid -> recpos -> data
#--------doc----------
# end of streamed data will be marked by "\x0"
# an doc 0-pos contain (in 0th page) start of doc
pagecount  : 8b
pagesize   : 3b (n : 1b * blocksz : 2b) = 16*4Kb=64kb
zid        : 4b+n*4b+"\x0"
#--------pag----------
# page in linar read initial look up will be sid
xsid:4b+2b+2b+2b+"\x0"
records
#---------------------


#---------------------------------------------
# index storage :
# - rule-1 :
# sorted continues sequence of nums
iid : 8b -> pid : 8b , sid : 4b
# o(1) look up with iidpos = iid * 20b
#  - rule-2 :
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


#---------------------------------------------
```
