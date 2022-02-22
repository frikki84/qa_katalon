<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GetTeqie by Id</name>
   <tag></tag>
   <elementGuidId>b3d6bd36-a5f5-4b8f-ac3a-e8475379ec29</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzUxMiJ9.eyJhdmF0YXJfdXJsIjoiaHR0cHM6Ly9saDMuZ29vZ2xldXNlcmNvbnRlbnQuY29tL2EtL0FPaDE0R2gtNWo1Uk82Zm9oMUtJcUE0NDQ4aXFLWlFTUDBHMUxQNDZnTkZvPXM5Ni1jIiwicm9sZXMiOlsiUk9MRV9URVFJRSIsIlJPTEVfU1VQRVJfQURNSU4iLCJST0xFX0FETUlOIiwiUk9MRV9NQU5BR0VSIiwiUk9MRV9TRVJWSUNFX0FETUlOIl0sImlzcyI6ImVtc19hdXRoX3NlcnZpY2UiLCJsYXN0X25hbWUiOiJEemlhZGtvdXNrYXlhIiwiZXhwIjoxNjQ1NDY0ODEyLCJmaXJzdF9uYW1lIjoiWWFuYSIsImVtYWlsIjoieWFuYS5kemlhZGtvdXNrYXlhQHNvZnRlcS5jb20ifQ.EjRh2g5_jLJxddwERQB5DCBfDAMHB5JCDZlT_9Y3WQLAq_SYvbUA_x2V6irVtiu-oPfYWs37yVRXINJHd_TnHtwXW6BOIpFEL0NddtOBO4NIWfGRRPqYfJOUJr2kSaUkApMCYheOGSQLvPjMLh_29a5j-js01bbI5_AJ0y4Dpy8NpuNHa3cw527HiMZDAweXh8Kd495M_Hvc90GmxW4Dp1jlQwqZh3LajQJNamyAaIk40nfLmB48AUSMIcoMxozqFIC7ssD9wlxuhSQybQofosdnCPqalA-qaB41RceCzSuVAJSYpsuZ28HR0wTL3zSFCoypElmOvGS6265zmQW5rC_Mrie0T1TfcwPNOnFf7XW5pGtkDvhgm4tnJZTeuYa9J_vN40sxv9wmVlbsUXFzfiZ6PoDJwaVmG97QTLKFqbnpuklwy3JNBxGNGbL-Bv6hZ30Y1LXP1cymKN7gc4dL_99hOWTkt9BEtnWUF9aUqZT7ZqS6eo6kgZqdGxmnlCzyeNxzUGZ6njMOzeDrxqMBF3IDSgm3F3VTt8THi3xLQUX9jmrsC-E_94PNmpQFwTdaQw7nheX0HcqWfQOXariveKs73Ut2KaOa2kKISAJhkBf3FCLUSANRbYDHIqEpdiXwoDeht8m-pOzMeZPMICaaxbwi1-7ixsXUi9w2HBgtRic</value>
   </httpHeaderProperties>
   <katalonVersion>8.2.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://localhost:8080/api/v1/teqies/384</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()





WS.verifyElementPropertyValue(response, 'firstName', 'Yana')



WS.verifyElementPropertyValue(response, 'office', 'BY')


</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
