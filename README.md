Convert content farm list
=========================

This is a tool to automatically convert the content farm list from [Content Farm Terminator](https://github.com/danny0838/content-farm-terminator)  to a uBO rule file.

Usage
-----

Please follow the [guide](https://github.com/gorhill/uBlock/wiki/Filter-lists-from-around-the-web) or the steps below to add the below list to your uBO filter list:

```
https://raw.githubusercontent.com/DanSnow/convert-content-farm-list/gh-pages/content-farm-list.txt
```

1. Open the uBO settings page.
2. Navigate to the `Filter Lists` tab.  
  ![image](https://user-images.githubusercontent.com/5575082/164973607-d0d4a0c6-c317-4053-8197-981eb9177cec.png)
3. Check the `Import` checkbox.  
  ![image](https://user-images.githubusercontent.com/5575082/164973673-66c4bdec-84a2-4a40-8e94-dc84e3bdca24.png)
4. Paste the above list into the `Import` textarea.  
  ![image](https://user-images.githubusercontent.com/5575082/164973701-06d0e546-b952-452e-ae52-7ce048bdb9a0.png)
5. Click the `Apply` button.  
  ![image](https://user-images.githubusercontent.com/5575082/164973712-e3063073-d6e5-4279-9174-e1006923ecc6.png)

Limitations
-----------

- Not support regex rule

TODO
----

- [ ] Don't update the list if there is no change
