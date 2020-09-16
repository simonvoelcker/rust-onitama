import React, { Component } from 'react'
import { AppConsumer, AppContext } from '../../context/AppContext'
import Cell from './Cell'

export default class Field extends Component {
  getCell (x, y) {
    let cellIndex = y*5+x
    let piece = this.context.state.game.field.pieces[cellIndex]
    return <Cell key={cellIndex} piece={piece} />
  }

  render () {
    return (
      <AppConsumer>
        {({ state, mutations }) => (
          <div className='field-panel'>
            <div className='field'>
              <div className='field-rows'>
              {[4,3,2,1,0].map((y) => {
                return (
                  <div key={y} className='field-columns'>
                  {[0,1,2,3,4].map((x) => this.getCell(x, y))}
                  </div>
                )
              })}
              </div>
            </div>
          </div>
        )}
      </AppConsumer>
    )
  }
}

Field.contextType = AppContext
