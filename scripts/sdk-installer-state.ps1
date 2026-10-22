# Read-only MSI component enumeration. Unlike the COM wrapper, distinguish
# an absent component from errors such as corrupt registration.
if (-not ('TokenDailyMsiState' -as [type])) {
    Add-Type -TypeDefinition @'
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Runtime.InteropServices;
using System.Text;
public static class TokenDailyMsiState {
    [DllImport("msi.dll", CharSet = CharSet.Unicode, ExactSpelling = true)]
    private static extern uint MsiEnumClientsW(string component, uint index, StringBuilder product);
    public static string[] Clients(string component) {
        var clients = new List<string>();
        for (uint index = 0; ; index++) {
            var product = new StringBuilder(39);
            uint result = MsiEnumClientsW(component, index, product);
            if (result == 259 || (result == 1607 && index == 0)) return clients.ToArray();
            if (result != 0) throw new Win32Exception((int)result);
            clients.Add(product.ToString());
        }
    }
}
'@
}
