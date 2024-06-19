import {TestStruct, create} from 'rust-lib'

const test = create()

TestStruct.foo() // will log "Foo" to the console
test.bar() // will log "Bar" to the console

console.log({TestStruct}) // will log the TestStruct class to the console