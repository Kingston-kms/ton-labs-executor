/*
* Copyright 2018-2020 TON DEV SOLUTIONS LTD.
*
* Licensed under the SOFTWARE EVALUATION License (the "License"); you may not use
* this file except in compliance with the License.  You may obtain a copy of the
* License at: https://ton.dev/licenses
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific TON DEV software governing permissions and
* limitations under the License.
*/

use ton_types::{Cell, SliceData};
use ton_vm::{
    executor::{Engine, gas::gas_state::Gas}, smart_contract_info::SmartContractInfo,
    stack::{Stack, StackItem, savelist::SaveList}
};

/// Builder for virtual machine engine. Initialises registers,
/// stack and code of VM engine. Returns initialized instance of TVM.
pub struct VMSetup {
    vm: Engine,
    code: SliceData,
    ctrls: SaveList,
    stack: Option<Stack>,
    gas: Option<Gas>,
}

impl VMSetup {
    /// Creates new instance of VMSetup with contract code.
    /// Initializes some registers of TVM with predefined values.
    pub fn new(code: SliceData) -> Self {
        VMSetup {
            vm: Engine::new(),
            code,
            ctrls: SaveList::new(),
            stack: None,
            gas: Some(Gas::empty()),
        }
    }

    /// Sets SmartContractInfo for TVM register c7
    pub fn set_contract_info(mut self, sci: &SmartContractInfo) -> VMSetup {
        self.ctrls.put(7, &mut sci.into_temp_data()).unwrap();
        self
    }

    /// Sets persistent data for contract in register c4
    pub fn set_data(mut self, data: Cell) -> VMSetup {
        self.ctrls.put(4, &mut StackItem::Cell(data)).unwrap();
        self
    }

    /// Sets initial stack for TVM
    pub fn set_stack(mut self, stack: Stack) -> VMSetup {
        self.stack = Some(stack);
        self
    }
    
    /// Sets gas for TVM
    pub fn set_gas(mut self, gas: Gas) -> VMSetup {
        self.gas = Some(gas);
        self
    }

    /// Sets trace flag to TVM for printing stack and commands
    #[allow(dead_code)]
    pub fn set_debug(mut self, enable: bool) -> VMSetup {
        if enable {
            self.vm.set_trace(Engine::TRACE_ALL);
        } else {
            self.vm.set_trace(0);
        }
        self
    }

    /// Creates new instance of TVM with defined stack, registers and code.
    pub fn create(self) -> Engine {
        self.vm.setup(self.code, Some(self.ctrls), self.stack, self.gas)
    }
}