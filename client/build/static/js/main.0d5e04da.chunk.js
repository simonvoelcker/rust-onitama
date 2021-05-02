(this["webpackJsonprust-onitama"]=this["webpackJsonprust-onitama"]||[]).push([[0],{55:function(e,t,n){},61:function(e,t,n){"use strict";n.r(t);var a=n(1),s=n.n(a),i=n(27),r=n.n(i),o=n(4),c=n(5),l=n(7),u=n(6),p=n(29),d=n(2),h=n(28),m=n.n(h),j=n(0),b=m.a.create({baseURL:"/api/",timeout:1e4,headers:{"Content-Type":"application/json"}}),f=s.a.createContext(),O=function(e){Object(l.a)(n,e);var t=Object(u.a)(n);function n(e){var a;return Object(o.a)(this,n),(a=t.call(this,e)).state={gameId:null,game:null,selection:{fromPosition:null,targetPosition:null,cardName:null},options:null,selectableOptions:[]},a.mutations={startNewGame:function(){return b.post("/games").then((function(e){a.setState({gameId:e.data}),a.mutations.getGame(e.data),a.mutations.getOptions(e.data)}))},getGame:function(e){return b.get("/games/"+e).then((function(e){a.setState({game:e.data})}))},getOptions:function(e){return b.get("/games/"+e+"/options").then((function(e){a.setState({options:e.data}),a.clearSelection()}))},pickOption:function(e){return b.post("/games/"+a.state.gameId+"/options/"+e).then((function(e){a.mutations.getGame(a.state.gameId),a.mutations.getOptions(a.state.gameId)}))},onCellClick:function(e,t){e&&e.player===a.state.game.current_player?a.setOrKeepSelection(t,null,null):a.setOrKeepSelection(null,t,null)},onCardClick:function(e){a.setOrKeepSelection(null,null,e)}},a}return Object(c.a)(n,[{key:"setOrKeepSelection",value:function(e,t,n){var a={fromPosition:e||this.state.selection.fromPosition,targetPosition:t||this.state.selection.targetPosition,cardName:n||this.state.selection.cardName};this.setSelection(a)}},{key:"setSelection",value:function(e){var t=this.filterSelectionOptions(e);if(t.length>0){if(this.setState({selection:e,selectableOptions:t}),null!==e.fromPosition&&null!==e.targetPosition&&null!==e.cardName&&1===t.length){var n=this.state.options.indexOf(t[0]);this.mutations.pickOption(n)}}else this.clearSelection()}},{key:"clearSelection",value:function(){this.setState({selection:{fromPosition:null,targetPosition:null,cardName:null},selectableOptions:this.state.options})}},{key:"filterSelectionOptions",value:function(e){return this.state.options.filter((function(t){return(null===e.fromPosition||t.from_position.x===e.fromPosition.x&&t.from_position.y===e.fromPosition.y)&&((null===e.targetPosition||t.target_position.x===e.targetPosition.x&&t.target_position.y===e.targetPosition.y)&&(null===e.cardName||t.card.name===e.cardName))}))}},{key:"render",value:function(){return Object(j.jsx)(f.Provider,{value:{state:this.state,mutations:this.mutations},children:this.props.children})}}]),n}(a.Component),y=f.Consumer,g=(n(54),n(55),function(e){Object(l.a)(n,e);var t=Object(u.a)(n);function n(){return Object(o.a)(this,n),t.apply(this,arguments)}return Object(c.a)(n,[{key:"getClasses",value:function(){var e=this.context.state.selection.fromPosition;if(e&&e.x===this.props.position.x&&e.y===this.props.position.y)return"cell from-cell";var t=this.context.state.selection.targetPosition;return t&&t.x===this.props.position.x&&t.y===this.props.position.y?"cell target-cell":"cell"}},{key:"getImageSrc",value:function(){return"/img/piece-"+(0===this.props.piece.player?"blue":"red")+"-"+(this.props.piece.is_master?"master":"apprentice")+"-front.png"}},{key:"render",value:function(){var e=this;return Object(j.jsx)(y,{children:function(t){t.state;var n=t.mutations;return Object(j.jsx)("div",{className:e.getClasses(),onClick:function(){return n.onCellClick(e.props.piece,e.props.position)},children:e.props.piece&&Object(j.jsx)("img",{className:"piece",src:e.getImageSrc(),alt:"piece"})})}})}}]),n}(a.Component));g.contextType=f;var v=function(e){Object(l.a)(n,e);var t=Object(u.a)(n);function n(){return Object(o.a)(this,n),t.apply(this,arguments)}return Object(c.a)(n,[{key:"getClasses",value:function(){var e="card";return this.context.state.selection.cardName===this.props.name&&(e+=" selected-card"),this.props.mini&&(e+=" mini-card"),e}},{key:"getImageSrc",value:function(){return"/img/card-"+this.props.name.toLowerCase()+".jpg"}},{key:"render",value:function(){var e=this;return Object(j.jsx)(y,{children:function(t){t.state;var n=t.mutations;return Object(j.jsx)("div",{className:"card-container",children:Object(j.jsx)("img",{onClick:function(){return n.onCardClick(e.props.name)},className:e.getClasses(),src:e.getImageSrc(),alt:"card"})})}})}}]),n}(a.Component);v.contextType=f;var x=function(e){Object(l.a)(n,e);var t=Object(u.a)(n);function n(){return Object(o.a)(this,n),t.apply(this,arguments)}return Object(c.a)(n,[{key:"getClasses",value:function(){return"0"===this.props.playerIndex?"player-panel":"player-panel opponent-player-panel"}},{key:"render",value:function(){var e=this;return Object(j.jsx)(y,{children:function(t){var n=t.state;return Object(j.jsxs)("div",{className:e.getClasses(),children:[parseInt(e.props.playerIndex)===n.game.current_player?Object(j.jsx)(v,{mini:!0,name:n.game.public_card.name}):Object(j.jsx)("div",{className:"mini-card"}),Object(j.jsxs)("div",{className:"cards",children:[Object(j.jsx)(v,{name:n.game.players[e.props.playerIndex].cards[0].name}),Object(j.jsx)(v,{name:n.game.players[e.props.playerIndex].cards[1].name})]})]})}})}}]),n}(a.Component);x.contextType=f;var w=function(e){Object(l.a)(n,e);var t=Object(u.a)(n);function n(){return Object(o.a)(this,n),t.apply(this,arguments)}return Object(c.a)(n,[{key:"getCell",value:function(e,t){var n=5*t+e,a=this.context.state.game.field.pieces[n],s={x:e,y:t};return Object(j.jsx)(g,{position:s,piece:a},n)}},{key:"render",value:function(){var e=this;return Object(j.jsx)(y,{children:function(t){t.state,t.mutations;return Object(j.jsx)("div",{className:"field-panel-wrapper",children:Object(j.jsxs)("div",{className:"field-panel",children:[Object(j.jsx)(x,{playerIndex:"1"}),Object(j.jsx)("div",{className:"field",children:Object(j.jsx)("div",{className:"field-rows",children:[4,3,2,1,0].map((function(t){return Object(j.jsx)("div",{className:"field-columns",children:[0,1,2,3,4].map((function(n){return e.getCell(n,t)}))},t)}))})}),Object(j.jsx)(x,{playerIndex:"0"})]})})}})}}]),n}(a.Component);w.contextType=f;var k=function(e){Object(l.a)(n,e);var t=Object(u.a)(n);function n(){return Object(o.a)(this,n),t.apply(this,arguments)}return Object(c.a)(n,[{key:"render",value:function(){return Object(j.jsx)(y,{children:function(e){e.state;var t=e.mutations;return Object(j.jsx)("div",{className:"dialog-wrapper",children:Object(j.jsxs)("div",{className:"dialog",children:[Object(j.jsx)("h1",{children:"Welcome!"}),Object(j.jsx)("p",{children:"This is a simple game called Onitama. I did not invent it, I merely implemented this online version of it as s hobby project."}),Object(j.jsx)("h2",{children:"How to play:"}),Object(j.jsx)("p",{children:"You are given control over a Kung Fu master and his apprentices."}),Object(j.jsx)("p",{children:Object(j.jsx)("b",{children:"Your goal is to either beat the enemy master with any of your pieces OR to reach the top middle square with your own master."})}),Object(j.jsxs)("p",{children:["You move according to the ",Object(j.jsx)("i",{children:"movement cards"})," presented on your player tableau:"]}),Object(j.jsx)("p",{children:Object(j.jsx)("b",{children:"Select one of your pieces, a movement card and a target square. The movement cards allow for different moves, as indicated by grey squares."})}),Object(j.jsx)("p",{children:"The target square must not be occupied by one of your own pieces already. After your turn, you lose the movement card you used but get the one that your opponent had used in his last turn. The five movement cards are randomly selected from sixteen existing ones, each representing a different style of Kung Fu."}),Object(j.jsx)("p",{children:Object(j.jsx)("b",{children:"Enjoy!"})}),Object(j.jsx)("button",{className:"btn",onClick:t.startNewGame,children:"Start Game"})]})})}})}}]),n}(a.Component);k.contextType=f;var C=function(e){Object(l.a)(n,e);var t=Object(u.a)(n);function n(){return Object(o.a)(this,n),t.apply(this,arguments)}return Object(c.a)(n,[{key:"getRandomMessage",value:function(e){var t=e?["The enemy didn't stand a chance, if we're honest.","You beat the computer on its own turf!","Bruce Lee himself couldn't have done it any better.","And what a finishing move it was. I'll never forget.","The enemy is probably scared of you now.","You came, you saw, you won.","Flawless execution, that's for sure.","That look on the enemy master's face, priceless!","I'm impressed. Can you do it again?","Nobody messes with you."]:["He probably only won because he was better than you.","That was super unfair. He had all the good cards.","But you fought well... At least I though so.","Well, at least you tried. Try again, though?","You were definitely ahead in the first half.","I did not see this coming, honestly.","You cannot give up halfway through. This is Kung Fu.","Your effort will be remembered, though.","It happened so quickly! How did he do that?","You learned something, that is what counts."];return t[Math.floor(Math.random()*t.length)]}},{key:"render",value:function(){var e=this,t=1===this.context.state.game.current_player;return Object(j.jsx)(y,{children:function(n){n.state;var a=n.mutations;return Object(j.jsx)("div",{className:"dialog-wrapper",children:Object(j.jsxs)("div",{className:"dialog",children:[Object(j.jsxs)("h1",{children:["You ",t?"won":"lost","!"]}),Object(j.jsx)("p",{children:e.getRandomMessage(t)}),Object(j.jsx)("button",{className:"btn",onClick:a.startNewGame,children:"Start new game"})]})})}})}}]),n}(a.Component);C.contextType=f;var N=function(e){Object(l.a)(n,e);var t=Object(u.a)(n);function n(){return Object(o.a)(this,n),t.apply(this,arguments)}return Object(c.a)(n,[{key:"render",value:function(){return Object(j.jsx)(y,{children:function(e){var t=e.state;e.mutations;return Object(j.jsxs)("div",{children:[null===t.game&&Object(j.jsx)(k,{}),null!==t.game&&Object(j.jsx)(w,{}),null!==t.game&&t.options&&0===t.options.length&&Object(j.jsx)(C,{})]})}})}}]),n}(a.Component);N.contextType=f;var I=function(e){Object(l.a)(n,e);var t=Object(u.a)(n);function n(){return Object(o.a)(this,n),t.apply(this,arguments)}return Object(c.a)(n,[{key:"render",value:function(){return Object(j.jsx)(O,{value:this.state,children:Object(j.jsx)(p.a,{children:Object(j.jsx)("div",{className:"App",children:Object(j.jsxs)(d.d,{children:[Object(j.jsx)(d.b,{path:"/",component:N,exact:!0}),Object(j.jsx)(d.b,{children:Object(j.jsx)(d.a,{to:"/"})})]})})})})}}]),n}(a.Component);r.a.render(Object(j.jsx)(I,{}),document.getElementById("root"))}},[[61,1,2]]]);
//# sourceMappingURL=main.0d5e04da.chunk.js.map