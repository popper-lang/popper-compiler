# Bytecode Format

## Function

```
FUNCTION_BYTECODE ( 0x01 )
  |
  ---> name: STR_VALUE_BYTECODE ( 0x01 )
  |
  ---> args: VEC_TYPES_VALUE_BYTECODE ( 0x02 )

```

## Type

```
Int ->      TYPES_INT_BYTECODE ( 0x01 )
Float ->    TYPES_FLOAT_BYTECODE ( 0x02 )
Bool ->     TYPES_BOOL_BYTECODE ( 0x03 )
String ->   TYPES_STRING_BYTECODE ( 0x04 )
                |-> len: USIZE_VALUE_BYTECODE ( 0x03 ) 
List ->     TYPES_LIST_BYTECODE ( 0x05 )
                |-> type: TYPES_BYTECODE ( 0x04 )
                |-> TYPES_ARRAY_NEXT_BYTECODE ( 0x01 )
                |-> len: USIZE_VALUE_BYTECODE ( 0x03 ) 

```

