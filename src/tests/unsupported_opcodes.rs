//!
//! The Solidity compiler unit tests for unsupported opcodes.
//!

#![cfg(test)]

use std::collections::BTreeMap;

use crate::solc::pipeline::Pipeline as SolcPipeline;

#[test]
#[should_panic(expected = "The `CODECOPY` instruction is not supported")]
fn codecopy_yul_runtime() {
    let source_code = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract FixedCodeCopy {
    function copyCode() public view returns (bytes memory) {
        uint256 fixedCodeSize = 64;
        bytes memory code = new bytes(fixedCodeSize);

        assembly {
            codecopy(add(code, 0x20), 0, fixedCodeSize)
        }

        return code;
    }
}
    "#;

    super::build_solidity(source_code, BTreeMap::new(), SolcPipeline::Yul).expect("Test failure");
}

pub const CALLCODE_TEST_SOURCE: &str = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract CallcodeTest {
    function testCallcode(address target, bytes4 signature, uint256 inputValue) public returns (bool) {
        bool success;

        assembly {
            let input := mload(0x40)
            mstore(input, signature)
            mstore(add(input, 0x04), inputValue)

            let callResult := callcode(gas(), target, 0, input, 0x24, 0, 0)

            success := and(callResult, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF)
        }

        return success;
    }
}
    "#;

#[test]
#[should_panic(expected = "The `CALLCODE` instruction is not supported")]
fn callcode_evmla() {
    super::build_solidity(CALLCODE_TEST_SOURCE, BTreeMap::new(), SolcPipeline::EVMLA)
        .expect("Test failure");
}

#[test]
#[should_panic(expected = "The `CALLCODE` instruction is not supported")]
fn callcode_yul() {
    super::build_solidity(CALLCODE_TEST_SOURCE, BTreeMap::new(), SolcPipeline::Yul)
        .expect("Test failure");
}

#[test]
#[should_panic(expected = "The `PC` instruction is not supported")]
fn pc_yul() {
    let source_code = r#"
object "ProgramCounter" {
    code {
        datacopy(0, dataoffset("ProgramCounter_deployed"), datasize("ProgramCounter_deployed"))
        return(0, datasize("ProgramCounter_deployed"))
    }
    object "ProgramCounter_deployed" {
        code {
            function getPC() -> programCounter {
                programCounter := pc()
            }

            let pcValue := getPC()
            sstore(0, pcValue)
        }
    }
}
    "#;

    super::build_yul(source_code).expect("Test failure");
}

pub const EXTCODECOPY_TEST_SOURCE: &str = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract ExternalCodeCopy {
    function copyExternalCode(address target, uint256 codeSize) public view returns (bytes memory) {
        bytes memory code = new bytes(codeSize);

        assembly {
            extcodecopy(target, add(code, 0x20), 0, codeSize)
        }

        return code;
    }
}
    "#;

#[test]
#[should_panic(expected = "The `EXTCODECOPY` instruction is not supported")]
fn extcodecopy_evmla() {
    super::build_solidity(
        EXTCODECOPY_TEST_SOURCE,
        BTreeMap::new(),
        SolcPipeline::EVMLA,
    )
    .expect("Test failure");
}

#[test]
#[should_panic(expected = "The `EXTCODECOPY` instruction is not supported")]
fn extcodecopy_yul() {
    super::build_solidity(EXTCODECOPY_TEST_SOURCE, BTreeMap::new(), SolcPipeline::Yul)
        .expect("Test failure");
}

pub const SELFDESTRUCT_TEST_SOURCE: &str = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract MinimalDestructible {
    address payable public owner;

    constructor() {
        owner = payable(msg.sender);
    }

    function destroy() public {
        require(msg.sender == owner, "Only the owner can call this function.");
        selfdestruct(owner);
    }
}
    "#;

#[test]
#[should_panic(expected = "The `SELFDESTRUCT` instruction is not supported")]
fn selfdestruct_evmla() {
    super::build_solidity(
        SELFDESTRUCT_TEST_SOURCE,
        BTreeMap::new(),
        SolcPipeline::EVMLA,
    )
    .expect("Test failure");
}

#[test]
#[should_panic(expected = "The `SELFDESTRUCT` instruction is not supported")]
fn selfdestruct_yul() {
    super::build_solidity(SELFDESTRUCT_TEST_SOURCE, BTreeMap::new(), SolcPipeline::Yul)
        .expect("Test failure");
}
