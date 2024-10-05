# oxdb-engine

> ## oxdb-engine - in Dev


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
# full uri flow structure which can point data in doc 
uri = db:doc:rid.pid.sid.recpos.data # o(1) 
# doc indexes
rid = ( pid , sid ) # rid used to pin point any rec in doc
iid  -> rid #o(1)      # iid is primary index point to rid        
uid  -> rid #o(log(n)) # uid is key secondery index point to rid

#---------------------------------------------
# struct expansio
db  = phy folder    #- entire database physical folder
doc = phy file      #- .oxd individual unit of database live in disk
record = phy data   #- a cantionus block of bin data in a doc
index = index data  #- .oxdi index file that stored all indexes
cache = cashe file  #- .oxdc pressisted inmemory data

#---------------------------------------------
# uri    expansion :
uri = reslocator   #- used as db level index to locate data
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
record  =   r : record  | i4b |  8 |keys_str+"\x00" | Data  |


# uri :
oxdbin  = Identifier    |size|    Data     |
uri     =   x : uri     | 4  |  str "\x00" |
del     =   n : delbyte | 4  |   "\x00"    | # may packed with "\x00" but ignored

# nums :
oxdbin  = Identifier    |      Data     | #no size rep direct data
int     =   i : integer | max = 8 bytes |
float   =   f : float   | max = 8 bytes |

# strings :
oxdbin  = Identifier    | Size | Data          |
str     =   s : string  |  4   |max(n) = "2^32" or "4gb"
large   =   l : large_s |  8   |max(n) = "2^64"bytes (theoretically)

# collections :
oxdbin  = Identifier    | Size | Data          |
vec     =   v : vector  |  4   | t any         |
map     =   m : hashmap |  4   | s str , t any |

# custom :
oxdbin  = Identifier    |size|cty| cData  |
cstm    =   c : custom  | 4  |str   | t any  | #Custom type

# uri units :
# starts with 4b resents size ends with "\x00"
oxdbin  = Identifier:size+n*cData+"\x00"
sid     = xsid:4b+4b+4b+4b+....+n*4b+"\x00"     # stream of bytes 
pid     = xpid:4b+4b+4b+4b+....+n*4b+"\x00"     # used to store lookups
zid     = xzid:4b+4b+4b+4b+....+n*4b+"\x00"     #- schema id or plan
iid     = xiid:4b+4b+4b+4b+....+n*4b+"\x00"     #- unique num id of rec
rid     = xrid:4b+10b+10b+.....+n*10b+"\x00"    #- (pid+sid)
uid     = xuid:str+str+........+n*str+"\x00"    #- (uuid or unique keystr named)


#---------------------------------------------
# db map :
# map representation of db 
# physical data rep
db : {
    doc : {
        pid : 4b = dynamic False : {
            pagesize  : psz (n 1b * blocksz 2b) = 16 * 4Kb = 64kb
            sid : 4b+..+n*4b+"\x00" True  ,
            rec : iid 4b : size 4b : uid nb+"\x00" : = dynamic True : {
                    data : type t : any = [ r , x , n , i , f , s , l , v , m , c ]
                }
        }
    }
}
# example in bin
basedb : {
    basedoc : {
        pagesize i 2 4096 xsid l 2 23 45 "\x00" r 101 10b "key1" "\x00" data r 256 "key2" "\x00" data
    }
}
# uri for above sample
uri = db:doc:1:2:rec
uri = db:doc:pid:sid:iid:uid:data.val
uri = db:doc:rid:iid:uid:data.val
uri = db:doc:rid:rec


#---------------------------------------------
# doc storage :
blocks = 4kb #default dependence on system hardware
pages  = n * blocks = 64kb # n = 16 default 
pages -> slotted pages [ slot-array -- , --> [records]]
pid -> sid -> recpos -> rec
#--------doc----------
# end of streamed data will be marked by "\x0"
# an doc 0-pos contain (in 0th page) start of doc
pagesize   : (n : 1b * blocksz : 2b) = 16*4Kb=64kb
#--------pag----------
# page in linar read initial look up will be sid
xsid:4b+2b+2b+2b+"\x00"
records = {
    i iid
    u uid
    d data
}
#---------------------


#---------------------------------------------
# index storage :
# - proto-1 :
# sorted continues sequence of nums
iid : 4b ->pid : 4b , sid : 4b
# o(1) look up with ridpos = iid * 10b
#  - proto-2 :
# Btree index
iid -> pid , sid
# o(log(n)) look up binary search


#---------------------------------------------
# Transaction :
#each opreation is Transc by default
begin edit
multiple operations
end commit
# flushed after commit to disk


#---------------------------------------------
```
