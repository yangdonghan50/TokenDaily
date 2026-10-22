import test from 'node:test';
import assert from 'node:assert/strict';
import { floatingIdentity } from '../src/domain/floatingIdentity.ts';
test('device identity stays guest by default and uses only a verified nickname',()=>{
 assert.equal(floatingIdentity(null).avatar,null);
 assert.equal(floatingIdentity({loggedIn:false,remote:{enrollment:{nickname:'Alice'}}}).avatar,null);
 assert.equal(floatingIdentity({loggedIn:true,remote:null}).label,'设备身份');
 assert.equal(floatingIdentity({loggedIn:true,remote:{enrollment:{nickname:'  东翰  '}}}).avatar,'东');
 assert.equal(floatingIdentity({loggedIn:true,remote:{enrollment:{nickname:'alice'}}}).avatar,'A');
 assert.equal(floatingIdentity({loggedIn:true,remote:{enrollment:null,account:{accountId:'account',nickname:'小明'}}}).label,'小明');
});
