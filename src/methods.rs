//! Many implementations to make stringlet easy to use.

use crate::*;

impl<Kind: StringletKind, const SIZE: usize> StringletBase<Kind, SIZE> {
    #[inline(always)]
    pub const fn as_bytes(&self) -> &[u8] {
        if Kind::FIXED {
            &self.str
        } else {
            // const equivalent of [..self.len()], asm differs in debug but same as slice in release
            //self.str.split_at(self.len()).0
            // SAFETY This is what String aka Vec uses and all bytes are initialized. This is 30% faster than split_at.
            unsafe { core::slice::from_raw_parts(self.str.as_ptr(), self.len()) }
        }
    }

    #[inline(always)]
    pub const fn as_str(&self) -> &str {
        // SAFETY: str up to len() is guaranteed to to be initialized with valid UTF-8
        unsafe { str::from_utf8_unchecked(self.as_bytes()) }
    }

    /* Once we add appending
    pub const fn capacity(&self) -> usize {
        SIZE
    } */

    #[inline(always)]
    pub const fn len(&self) -> usize {
        // optimizer should elide all but one if-branch as all conditions are const
        if Kind::FIXED || SIZE == 0 {
            return SIZE;
        }

        let last = self.last();
        if Kind::VAR {
            // For VarStringlet look no further
            last as _
        } else if SIZE == 1 {
            // iff single byte is not TAG | 1 we have 1 (use +, because mutants flags ^, which in this case is identical to |)
            (last != TAG + 1) as _
        } else if Kind::TRIM {
            // branchless: if last is tagged, subtract one
            SIZE - (last == TAG + 1) as usize
        }
        // Next two SlimStringlet
        else if SIZE == 64 {
            // 64 is special as we only store 6 bits, where 0b00_0000 means SIZE-0b100_0000
            SIZE - (last == TAG) as usize * SIZE - (last > TAG) as usize * (last ^ TAG) as usize
        } else {
            // branchless: if last is UTF-8, SIZE, else extract tail len from low bits of last
            SIZE - (last > TAG) as usize * (last ^ TAG) as usize
        }
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        if SIZE == 0 {
            // trivially
            true
        } else if Kind::FIXED {
            // FIXED > 0 can never be empty
            false
        } else if Kind::VAR {
            self.last() == 0
        } else if Kind::TRIM {
            // TRIM is only empty if last is tagged
            SIZE == 1 && self.last() == TAG + 1
        }
        // Next two SlimStringlet
        else if SIZE == 64 {
            // Special case as we have 65 lengths but only 6 bits.
            self.last() == TAG
        } else {
            self.last() == TAG + SIZE as u8
        }
    }

    #[inline(always)]
    pub(crate) const fn last(&self) -> u8 {
        if Kind::VAR {
            self.var_last()
        } else {
            debug_assert!(SIZE != 0, "unchecked call");
            self.str[SIZE - 1]
        }
    }
}
