@[extern "arr"]
opaque addFromCpp : UInt32 → UInt32 → Array String

-- @[extern "add_from_rust"]
-- opaque addFromRust : UInt32 → UInt32 → UInt32

def main : IO Unit := do
  IO.println $ (addFromCpp 1 2)
  -- IO.println $ (addFromCpp 1 2)
  -- IO.println $ (addFromCpp 1 2).size
  -- IO.println $ (addFromRust 1 2)
