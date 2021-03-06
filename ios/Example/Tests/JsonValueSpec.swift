/**
 * Copyright (c) Visly Inc.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Quick
import Nimble
@testable import ShardKit

class JsonValueSpec: QuickSpec {
    override func spec() {
        it("should convert nil to JSON.Null") {
            let json = try! JsonValue(["value": nil]).asObject()
            json.get("value", {
                switch $0 {
                case .Null: ()
                default: fail("Expected object")
                }
            })
        }

        it("should convert Int to JSON.Number") {
            let json = try! JsonValue(["value": Int(0)]).asObject()
            json.get("value", {
                switch $0 {
                case .Number: ()
                default: fail("Expected number")
                }
            })
        }

        it("should convert Float to JSON.Number") {
            let json = try! JsonValue(["value": Float(0)]).asObject()
            json.get("value", {
                switch $0 {
                case .Number: ()
                default: fail("Expected number")
                }
            })
        }
        
        it("should convert Double to JSON.Number") {
            let json = try! JsonValue(["value": Double(0)]).asObject()
            json.get("value", {
                switch $0 {
                case .Number: ()
                default: fail("Expected number")
                }
            })
        }
        
        it("should convert bool to JSON.Boolean") {
            let json = try! JsonValue(["value": true]).asObject()
            json.get("value", {
                switch $0 {
                case .Boolean: ()
                default: fail("Expected boolean")
                }
            })
        }
        
        it("should convert string to JSON.String") {
            let json = try! JsonValue(["value": ""]).asObject()
            json.get("value", {
                switch $0 {
                case .String: ()
                default: fail("Expected string")
                }
            })
        }
        
        it("should convert dictionary to JSON.Object") {
            let json = try! JsonValue(["value": [:]]).asObject()
            json.get("value", {
                switch $0 {
                case .Object: ()
                default: fail("Expected object")
                }
            })
        }

        it("should convert array to JSON.Array") {
            let json = try! JsonValue(["value": []]).asObject()
            json.get("value", {
                switch $0 {
                case .Array: ()
                default: fail("Expected array")
                }
            })
        }
    }
}
